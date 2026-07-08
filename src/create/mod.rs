pub mod git;
pub mod project;
pub mod template;

use std::io::{self, Write};

use anyhow::Result as AnyhowResult;
use console::style;
use serde::{Deserialize, Serialize};

use crate::error::OdkError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectConfig {
    pub project_name: String,
    pub git_repository: String,
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
    pub git_repository: String,
    pub odoo_version: String,
    pub python_version: String,
    pub postgres_version: String,
    pub use_docker: bool,
    pub generate_pycharm: bool,
    pub generate_vscode: bool,
}

impl ProjectConfig {
    pub fn from_options(options: ProjectOptions) -> Result<Self, OdkError> {
        validate_python_version(&options.odoo_version, &options.python_version)?;
        let database_name = database_name_from_project(&options.project_name);

        Ok(Self {
            project_name: options.project_name,
            git_repository: options.git_repository,
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
    let git_repository = prompt_required("Git Repository")?;
    let odoo_version = prompt_choice("Odoo Version", &["19.0", "18.0", "17.0"])?;
    let python_version = prompt_choice("Python Version", &["3.13", "3.12", "3.11"])?;
    let postgres_version = prompt_choice("PostgreSQL Version", &["17", "16"])?;
    let use_docker = prompt_yes_no("Use Docker")?;
    let generate_pycharm = prompt_yes_no("Generate PyCharm")?;
    let generate_vscode = prompt_yes_no("Generate VS Code")?;

    let options = ProjectOptions {
        project_name,
        git_repository,
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

pub fn validate_python_version(odoo_version: &str, python_version: &str) -> Result<(), OdkError> {
    let supported = supported_python_versions(odoo_version);
    if supported.contains(&python_version) {
        return Ok(());
    }

    Err(OdkError::InvalidPythonVersion {
        odoo_version: odoo_version.to_owned(),
        python_version: python_version.to_owned(),
        supported: supported.join(", "),
    })
}

pub fn supported_python_versions(odoo_version: &str) -> &'static [&'static str] {
    match odoo_version {
        "19.0" => &["3.12", "3.13"],
        "18.0" => &["3.11", "3.12"],
        "17.0" => &["3.11"],
        _ => &[],
    }
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

fn prompt_choice(label: &str, choices: &[&str]) -> AnyhowResult<String> {
    loop {
        println!("{label}:");
        for choice in choices {
            println!("  {choice}");
        }
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let value = input.trim();

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
    use super::{database_name_from_project, supported_python_versions, validate_python_version};

    #[test]
    fn validates_python_compatibility() {
        assert!(validate_python_version("19.0", "3.13").is_ok());
        assert!(validate_python_version("18.0", "3.11").is_ok());
        assert!(validate_python_version("19.0", "3.11").is_err());
    }

    #[test]
    fn exposes_supported_python_versions() {
        assert_eq!(supported_python_versions("19.0"), &["3.12", "3.13"]);
        assert_eq!(supported_python_versions("18.0"), &["3.11", "3.12"]);
    }

    #[test]
    fn normalizes_database_name() {
        assert_eq!(
            database_name_from_project("GEAAI Odoo!"),
            "geaai_odoo".to_owned()
        );
        assert_eq!(database_name_from_project("!!!"), "odoo".to_owned());
    }
}
