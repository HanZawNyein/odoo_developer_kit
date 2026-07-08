use std::path::Path;
use std::process::Command;

use crate::error::OdkError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    pub program: String,
    pub args: Vec<String>,
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    pub fn success(&self) -> bool {
        self.status == Some(0)
    }

    pub fn command_line(&self) -> String {
        let mut parts = Vec::with_capacity(self.args.len() + 1);
        parts.push(self.program.clone());
        parts.extend(self.args.iter().cloned());
        parts.join(" ")
    }

    pub fn combined_output(&self) -> String {
        let stdout = self.stdout.trim();
        if stdout.is_empty() {
            self.stderr.trim().to_owned()
        } else {
            stdout.to_owned()
        }
    }
}

pub fn run_command(
    program: &str,
    args: &[String],
    cwd: Option<&Path>,
) -> Result<CommandOutput, OdkError> {
    let mut command = Command::new(program);
    command.args(args);

    if let Some(directory) = cwd {
        command.current_dir(directory);
    }

    match command.output() {
        Ok(output) => Ok(CommandOutput {
            program: program.to_owned(),
            args: args.to_vec(),
            status: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Err(OdkError::CommandNotFound(program.to_owned()))
        }
        Err(error) => Err(OdkError::Io(error)),
    }
}

pub fn run_required(program: &str, args: &[String], cwd: Option<&Path>) -> Result<(), OdkError> {
    let output = run_command(program, args, cwd)?;
    if output.success() {
        return Ok(());
    }

    let stderr = output.combined_output();
    Err(OdkError::CommandFailed {
        command: output.command_line(),
        status: output
            .status
            .map_or_else(|| "unknown".to_owned(), |status| status.to_string()),
        stderr: if stderr.is_empty() {
            "command failed without output".to_owned()
        } else {
            stderr
        },
    })
}

pub fn first_version(text: &str) -> Option<String> {
    let mut version = String::new();
    let mut started = false;
    let mut has_digit = false;

    for character in text.chars() {
        if character.is_ascii_digit() {
            version.push(character);
            started = true;
            has_digit = true;
        } else if started && character == '.' {
            version.push(character);
        } else if started {
            break;
        }
    }

    let trimmed = version.trim_matches('.').to_owned();
    if has_digit && !trimmed.is_empty() {
        Some(trimmed)
    } else {
        None
    }
}

pub fn string_args(args: &[&str]) -> Vec<String> {
    args.iter().map(|arg| (*arg).to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use super::first_version;

    #[test]
    fn extracts_first_version() {
        assert_eq!(first_version("uv 0.8.4"), Some("0.8.4".to_owned()));
        assert_eq!(
            first_version("psql (PostgreSQL) 17.5"),
            Some("17.5".to_owned())
        );
        assert_eq!(first_version("no version"), None);
    }
}
