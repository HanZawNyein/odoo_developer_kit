pub mod postgres;
pub mod pycharm;
pub mod uv;
pub mod vscode;
pub mod wkhtmltopdf;

use anyhow::Result;
use console::style;

use crate::utils::command::{first_version, run_command, string_args};
use crate::utils::platform::{Platform, install_suggestion};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    pub name: &'static str,
    pub installed: bool,
    pub version: Option<String>,
    pub suggestion: Option<String>,
}

impl CheckResult {
    pub fn installed(name: &'static str, version: Option<String>) -> Self {
        Self {
            name,
            installed: true,
            version,
            suggestion: None,
        }
    }

    pub fn missing(name: &'static str, platform: Platform) -> Self {
        Self {
            name,
            installed: false,
            version: None,
            suggestion: Some(install_suggestion(name, platform).to_owned()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CommandSpec<'a> {
    pub program: &'a str,
    pub args: &'a [&'a str],
}

pub fn run() -> Result<()> {
    let platform = Platform::detect();
    let checks = run_checks(platform);
    render_checks(&checks, platform);
    Ok(())
}

pub fn run_checks(platform: Platform) -> Vec<CheckResult> {
    vec![
        uv::check(platform),
        pycharm::check(platform),
        vscode::check(platform),
        postgres::check(platform),
        wkhtmltopdf::check(platform),
    ]
}

pub fn command_check(
    name: &'static str,
    candidates: &[CommandSpec<'_>],
    platform: Platform,
) -> CheckResult {
    for candidate in candidates {
        let args = string_args(candidate.args);
        let output = match run_command(candidate.program, &args, None) {
            Ok(output) => output,
            Err(_) => continue,
        };

        if output.success() {
            let version = version_from_output(&output.combined_output());
            return CheckResult::installed(name, version);
        }
    }

    CheckResult::missing(name, platform)
}

fn render_checks(checks: &[CheckResult], platform: Platform) {
    println!("{}", style("Odoo Developer Kit Doctor").bold());
    println!();

    for check in checks {
        if check.installed {
            println!("[{}] {}", style("✓").green(), style(check.name).bold());
            println!(
                "    Version: {}",
                check.version.as_deref().unwrap_or("Unknown")
            );
        } else {
            println!("[{}] {}", style("✗").red(), style(check.name).bold());
            println!("    Not installed");
            if let Some(suggestion) = &check.suggestion {
                println!();
                println!("{}", style("Suggestion:").bold());
                println!("    {suggestion}");
            }
        }
        println!();
    }

    if checks.iter().all(|check| check.installed) {
        println!("{}", style("Environment ready!").green().bold());
    } else {
        println!(
            "{}",
            style(format!(
                "Environment needs attention on {}.",
                platform.name()
            ))
            .yellow()
            .bold()
        );
    }
}

pub fn version_from_output(output: &str) -> Option<String> {
    first_version(output).or_else(|| {
        output
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .map(str::to_owned)
    })
}

#[cfg(test)]
mod tests {
    use crate::doctor::version_from_output;

    #[test]
    fn parses_version_from_doctor_output() {
        assert_eq!(
            version_from_output("Python 3.13.5"),
            Some("3.13.5".to_owned())
        );
        assert_eq!(
            version_from_output("1.104.0\ncommit-hash"),
            Some("1.104.0".to_owned())
        );
    }
}
