use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs};

use anyhow::Result;
use indicatif::ProgressBar;

use crate::create::ProjectConfig;
use crate::create::git;
use crate::create::template::TemplateRenderer;
use crate::error::OdkError;
use crate::utils::command::{run_command, run_required};

pub fn create_project(config: &ProjectConfig) -> Result<PathBuf> {
    let target = absolute_path(&PathBuf::from(&config.project_path))?;
    ensure_target_is_available(&target)?;

    if config.git_repository.trim().is_empty() {
        fs::create_dir_all(&target)?;
    } else {
        run_step(
            "Cloning repository",
            Some(config.git_repository.clone()),
            || git::clone_repository(&config.git_repository, &target),
        )?;
    }

    run_step("Installing Python with uv", None, || {
        let args = vec![
            "python".to_owned(),
            "install".to_owned(),
            config.python_version.clone(),
        ];
        run_required("uv", &args, Some(&target))
    })?;

    run_step(
        "Creating virtual environment with uv",
        Some(target.join(".venv").display().to_string()),
        || {
            let args = vec![
                "venv".to_owned(),
                ".venv".to_owned(),
                "--python".to_owned(),
                config.python_version.clone(),
            ];
            run_required("uv", &args, Some(&target))
        },
    )?;

    run_ignored_step(
        "Installing Odoo requirements with uv",
        Some(
            odoo_requirements_path(&config.odoo_source_path)
                .display()
                .to_string(),
        ),
        || {
            let args = vec![
                "pip".to_owned(),
                "install".to_owned(),
                "-r".to_owned(),
                odoo_requirements_path(&config.odoo_source_path)
                    .display()
                    .to_string(),
            ];
            run_command("uv", &args, Some(&target))
        },
    );

    create_project_directories(&target, config.use_docker)?;

    let renderer = TemplateRenderer::new()?;
    let render_config = ProjectConfig {
        project_path: target.display().to_string(),
        ..config.clone()
    };
    renderer.render_project(&render_config, &target)?;

    println!(
        "Local addons_path: {}/addons,{}/addons",
        render_config.odoo_source_path, render_config.project_path
    );

    run_step(
        "Generating uv.lock",
        Some(target.join("uv.lock").display().to_string()),
        || {
            let args = vec!["lock".to_owned()];
            run_required("uv", &args, Some(&target))
        },
    )?;

    Ok(target)
}

fn run_step<F>(message: &'static str, success_detail: Option<String>, action: F) -> Result<()>
where
    F: FnOnce() -> Result<(), OdkError>,
{
    let progress = ProgressBar::new_spinner();
    progress.enable_steady_tick(Duration::from_millis(80));
    progress.set_message(message.to_owned());

    match action() {
        Ok(()) => {
            let done_message = match success_detail {
                Some(detail) => format!("{message} done - {detail}"),
                None => format!("{message} done"),
            };
            progress.finish_with_message(done_message);
            Ok(())
        }
        Err(error) => {
            progress.abandon_with_message(format!("{message} failed"));
            Err(error.into())
        }
    }
}

fn run_ignored_step<F>(message: &'static str, success_detail: Option<String>, action: F)
where
    F: FnOnce() -> Result<crate::utils::command::CommandOutput, OdkError>,
{
    let progress = ProgressBar::new_spinner();
    progress.enable_steady_tick(Duration::from_millis(80));
    progress.set_message(message.to_owned());

    match action() {
        Ok(output) if output.success() => {
            let done_message = match success_detail {
                Some(detail) => format!("{message} done - {detail}"),
                None => format!("{message} done"),
            };
            progress.finish_with_message(done_message);
        }
        Ok(output) => {
            progress.abandon_with_message(format!(
                "{message} failed; continuing so dependencies can be fixed manually"
            ));
            let output_text = output.combined_output();
            if !output_text.is_empty() {
                eprintln!("{output_text}");
            }
        }
        Err(error) => {
            progress.abandon_with_message(format!(
                "{message} failed; continuing so dependencies can be fixed manually"
            ));
            eprintln!("{error}");
        }
    }
}

fn odoo_requirements_path(odoo_source_path: &str) -> PathBuf {
    PathBuf::from(odoo_source_path).join("requirements.txt")
}

fn absolute_path(path: &Path) -> Result<PathBuf, OdkError> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()?.join(path))
    }
}

fn ensure_target_is_available(target: &Path) -> Result<(), OdkError> {
    if target.exists() {
        let mut entries = fs::read_dir(target)?;
        if entries.next().transpose()?.is_some() {
            return Err(OdkError::NonEmptyTarget(target.to_path_buf()));
        }
    }

    if let Some(parent) = target.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}

fn create_project_directories(target: &Path, use_docker: bool) -> Result<(), OdkError> {
    let mut directories = vec!["addons"];
    if use_docker {
        directories.push("config");
    }

    for directory in directories {
        fs::create_dir_all(target.join(directory))?;
    }
    Ok(())
}
