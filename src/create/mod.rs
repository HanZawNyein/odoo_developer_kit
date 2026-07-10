pub mod git;
pub mod project;
pub mod template;

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use anyhow::Result as AnyhowResult;
use console::style;
use serde::{Deserialize, Serialize};

use crate::error::OdkError;

const ODOO_VERSIONS: &[&str] = &["19.1", "19.0", "18.1", "18.0", "17.0"];
const PYTHON_VERSIONS: &[&str] = &["3.8", "3.9", "3.10", "3.11", "3.12", "3.13"];
const POSTGRES_VERSIONS: &[&str] = &["17", "16"];
const DEFAULT_ODOO_VERSION: &str = "19.0";
const DEFAULT_PYTHON_VERSION: &str = "3.10";
const DEFAULT_POSTGRES_VERSION: &str = "17";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectConfig {
    pub project_name: String,
    pub project_path: String,
    pub git_repository: String,
    pub odoo_source_path: String,
    pub odoo_version: String,
    pub python_version: String,
    pub postgres_version: String,
    pub database_name: String,
    pub use_docker: bool,
    pub generate_pycharm: bool,
    pub generate_vscode: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectOptions {
    pub project_name: String,
    pub project_path: String,
    pub git_repository: String,
    pub odoo_source_path: String,
    pub odoo_version: String,
    pub python_version: String,
    pub postgres_version: String,
    pub use_docker: bool,
    pub generate_pycharm: bool,
    pub generate_vscode: bool,
}

impl ProjectConfig {
    pub fn from_options(options: ProjectOptions) -> Result<Self, OdkError> {
        validate_python_version(&options.python_version)?;
        let database_name = database_name_from_project(&options.project_name);
        let project_path = project_path_from_input(&options.project_path, &options.project_name);

        Ok(Self {
            project_name: options.project_name,
            project_path,
            git_repository: options.git_repository,
            odoo_source_path: options.odoo_source_path,
            odoo_version: options.odoo_version,
            python_version: options.python_version,
            postgres_version: options.postgres_version,
            database_name,
            use_docker: options.use_docker,
            generate_pycharm: options.generate_pycharm,
            generate_vscode: options.generate_vscode,
        })
    }
}

pub fn run() -> AnyhowResult<()> {
    println!("{}", style("Odoo Project Creator").bold());
    println!();

    let config = prompt_project_config()?;
    let path = project::create_project(&config)?;

    println!();
    println!(
        "{} {}",
        style("Created Odoo project at").green().bold(),
        path.display()
    );
    Ok(())
}

pub fn prompt_project_config() -> AnyhowResult<ProjectConfig> {
    let project_name = prompt_required("Project Name")?;
    let project_path = prompt_text_with_default("Project Path", &project_name)?;
    let git_repository = prompt_optional("Git Repository")?;
    let odoo_source_path = prompt_required("Odoo Source Code Path")?;
    let odoo_version = prompt_odoo_version(&odoo_source_path)?;
    let python_version =
        prompt_choice_with_default("Python Version", PYTHON_VERSIONS, DEFAULT_PYTHON_VERSION)?;
    let use_docker = prompt_yes_no("Use Docker")?;
    let postgres_version = if use_docker {
        prompt_choice_with_default(
            "PostgreSQL Version",
            POSTGRES_VERSIONS,
            DEFAULT_POSTGRES_VERSION,
        )?
    } else {
        DEFAULT_POSTGRES_VERSION.to_owned()
    };
    let generate_pycharm = prompt_yes_no("Generate PyCharm")?;
    let generate_vscode = prompt_yes_no("Generate VS Code")?;

    let options = ProjectOptions {
        project_name,
        project_path,
        git_repository,
        odoo_source_path,
        odoo_version,
        python_version,
        postgres_version,
        use_docker,
        generate_pycharm,
        generate_vscode,
    };
    let config = ProjectConfig::from_options(options)?;
    Ok(config)
}

pub fn validate_python_version(python_version: &str) -> Result<(), OdkError> {
    let supported = supported_python_versions();
    if supported.contains(&python_version) {
        return Ok(());
    }

    Err(OdkError::InvalidPythonVersion {
        python_version: python_version.to_owned(),
        supported: supported.join(", "),
    })
}

pub fn supported_python_versions() -> &'static [&'static str] {
    PYTHON_VERSIONS
}

pub fn detect_odoo_version_from_source(odoo_source_path: &str) -> Option<String> {
    let release_path = Path::new(odoo_source_path.trim())
        .join("odoo")
        .join("release.py");
    let release = fs::read_to_string(release_path).ok()?;
    detect_odoo_version_from_release(&release)
}

fn detect_odoo_version_from_release(release: &str) -> Option<String> {
    for line in release.lines() {
        let line = line.trim();
        if !line.starts_with("version_info") {
            continue;
        }

        let (_, value) = line.split_once('=')?;
        let tuple = value.trim().strip_prefix('(')?.split_once(')')?.0;
        let mut parts = tuple.split(',').map(str::trim);
        let major = parts.next()?;
        let minor = parts.next()?;

        if let (Ok(major), Ok(minor)) = (major.parse::<u16>(), minor.parse::<u16>()) {
            return Some(format!("{major}.{minor}"));
        }

        if let Some(version) = first_major_minor_version(major) {
            return Some(version);
        }
    }

    None
}

pub fn docker_odoo_version(odoo_version: &str) -> String {
    match odoo_version.trim().split_once('.') {
        Some((major, _)) if !major.is_empty() && major.chars().all(|c| c.is_ascii_digit()) => {
            format!("{major}.0")
        }
        _ => odoo_version.to_owned(),
    }
}

fn first_major_minor_version(text: &str) -> Option<String> {
    let mut characters = text.trim_matches(['"', '\'']).chars().peekable();

    while let Some(character) = characters.next() {
        if !character.is_ascii_digit() {
            continue;
        }

        let mut major = String::from(character);
        while let Some(next) = characters.peek() {
            if next.is_ascii_digit() {
                major.push(*next);
                characters.next();
            } else {
                break;
            }
        }

        if characters.next() != Some('.') {
            continue;
        }

        let mut minor = String::new();
        while let Some(next) = characters.peek() {
            if next.is_ascii_digit() {
                minor.push(*next);
                characters.next();
            } else {
                break;
            }
        }

        if !minor.is_empty() {
            return Some(format!("{major}.{minor}"));
        }
    }

    None
}

pub fn database_name_from_project(project_name: &str) -> String {
    let mut database_name = String::new();
    let mut previous_was_separator = false;

    for character in project_name.trim().chars() {
        if character.is_ascii_alphanumeric() {
            database_name.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !previous_was_separator {
            database_name.push('_');
            previous_was_separator = true;
        }
    }

    let trimmed = database_name.trim_matches('_').to_owned();
    if trimmed.is_empty() {
        "odoo".to_owned()
    } else {
        trimmed
    }
}

pub fn project_path_from_input(project_path: &str, project_name: &str) -> String {
    let trimmed = project_path.trim();
    let project_name = project_name.trim();

    if trimmed.is_empty() {
        return project_name.to_owned();
    }

    let path = Path::new(trimmed);
    if path.file_name().is_some_and(|name| name == project_name) {
        trimmed.to_owned()
    } else {
        path.join(project_name).to_string_lossy().into_owned()
    }
}

fn prompt_required(label: &str) -> AnyhowResult<String> {
    loop {
        println!("{label}:");
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let value = input.trim().to_owned();

        if !value.is_empty() {
            println!();
            return Ok(value);
        }

        println!("{}", style("Value is required.").red());
    }
}

fn prompt_text_with_default(label: &str, default: &str) -> AnyhowResult<String> {
    println!("{label}:");
    println!("  {default} (default)");
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let value = input.trim();

    println!();
    if value.is_empty() {
        Ok(default.to_owned())
    } else {
        Ok(value.to_owned())
    }
}

fn prompt_optional(label: &str) -> AnyhowResult<String> {
    println!("{label}:");
    println!("  leave empty to skip");
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!();
    Ok(input.trim().to_owned())
}

fn prompt_choice_with_default(
    label: &str,
    choices: &[&str],
    default: &str,
) -> AnyhowResult<String> {
    debug_assert!(choices.contains(&default));
    prompt_choice_inner(label, choices, Some(default))
}

fn prompt_odoo_version(odoo_source_path: &str) -> AnyhowResult<String> {
    if let Some(version) = detect_odoo_version_from_source(odoo_source_path) {
        println!("Odoo Version:");
        println!("  {version} (detected)");
        println!();
        return Ok(version);
    }

    prompt_choice_with_default("Odoo Version", ODOO_VERSIONS, DEFAULT_ODOO_VERSION)
}

fn prompt_choice_inner(
    label: &str,
    choices: &[&str],
    default: Option<&str>,
) -> AnyhowResult<String> {
    loop {
        println!("{label}:");
        for choice in choices {
            if Some(*choice) == default {
                println!("  {choice} (default)");
            } else {
                println!("  {choice}");
            }
        }
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let value = input.trim();

        if value.is_empty()
            && let Some(default) = default
        {
            println!();
            return Ok(default.to_owned());
        }

        if choices.contains(&value) {
            println!();
            return Ok(value.to_owned());
        }

        println!(
            "{} {}",
            style("Invalid choice. Expected one of:").red(),
            choices.join(", ")
        );
    }
}

fn prompt_yes_no(label: &str) -> AnyhowResult<bool> {
    loop {
        println!("{label}:");
        println!("  yes/no");
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let value = input.trim().to_ascii_lowercase();

        match value.as_str() {
            "y" | "yes" => {
                println!();
                return Ok(true);
            }
            "n" | "no" => {
                println!();
                return Ok(false);
            }
            _ => println!("{}", style("Enter yes or no.").red()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        database_name_from_project, detect_odoo_version_from_source, docker_odoo_version,
        project_path_from_input, supported_python_versions, validate_python_version,
    };
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn validates_python_compatibility() {
        assert!(validate_python_version("3.8").is_ok());
        assert!(validate_python_version("3.10").is_ok());
        assert!(validate_python_version("3.13").is_ok());
        assert!(validate_python_version("3.7").is_err());
    }

    #[test]
    fn exposes_supported_python_versions() {
        assert_eq!(
            supported_python_versions(),
            &["3.8", "3.9", "3.10", "3.11", "3.12", "3.13"]
        );
    }

    #[test]
    fn detects_odoo_version_from_source_path() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be monotonic")
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("odk-odoo-source-{unique}"));
        let odoo_dir = temp_dir.join("odoo");
        fs::create_dir_all(&odoo_dir).expect("odoo dir should be created");
        fs::write(
            odoo_dir.join("release.py"),
            "RELEASE_LEVELS = [FINAL] = ['final']\nversion_info = (19, 0, 0, FINAL, 0, '')\n",
        )
        .expect("release file should be written");

        assert_eq!(
            detect_odoo_version_from_source(&temp_dir.to_string_lossy()),
            Some("19.0".to_owned())
        );
    }

    #[test]
    fn detects_minor_odoo_version_from_source_path() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be monotonic")
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("odk-odoo-saas-source-{unique}"));
        let odoo_dir = temp_dir.join("odoo");
        fs::create_dir_all(&odoo_dir).expect("odoo dir should be created");
        fs::write(
            odoo_dir.join("release.py"),
            "RELEASE_LEVELS = [FINAL] = ['final']\nversion_info = ('saas~18.1', 0, 0, FINAL, 0, '')\n",
        )
        .expect("release file should be written");

        assert_eq!(
            detect_odoo_version_from_source(&temp_dir.to_string_lossy()),
            Some("18.1".to_owned())
        );
    }

    #[test]
    fn derives_stable_docker_tag_from_odoo_version() {
        assert_eq!(docker_odoo_version("18.0"), "18.0");
        assert_eq!(docker_odoo_version("18.1"), "18.0");
        assert_eq!(docker_odoo_version("19.1"), "19.0");
    }

    #[test]
    fn normalizes_database_name() {
        assert_eq!(
            database_name_from_project("GEAAI Odoo!"),
            "geaai_odoo".to_owned()
        );
        assert_eq!(database_name_from_project("!!!"), "odoo".to_owned());
    }

    #[test]
    fn resolves_project_path_from_parent_directory() {
        assert_eq!(project_path_from_input("", "sample"), "sample".to_owned());
        assert_eq!(
            project_path_from_input("sample", "sample"),
            "sample".to_owned()
        );
        assert_eq!(
            project_path_from_input("projects", "sample"),
            "projects/sample".to_owned()
        );
        assert_eq!(
            project_path_from_input(" /Users/agga/Documents/python-dev/odoo-dev ", "sample"),
            "/Users/agga/Documents/python-dev/odoo-dev/sample".to_owned()
        );
        assert_eq!(
            project_path_from_input(
                " /Users/agga/Documents/python-dev/odoo-dev/sample ",
                "sample"
            ),
            "/Users/agga/Documents/python-dev/odoo-dev/sample".to_owned()
        );
    }
}
