use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs};

use anyhow::Result;
use indicatif::ProgressBar;

use crate::create::ProjectConfig;
use crate::create::git;
use crate::create::template::TemplateRenderer;
use crate::error::OdkError;
use crate::utils::command::run_required;

pub fn create_project(config: &ProjectConfig) -> Result<PathBuf> {
    let target = absolute_path(&PathBuf::from(&config.project_path))?;
    ensure_target_is_available(&target)?;

    run_step(
        "Cloning repository",
        Some(config.git_repository.clone()),
        || git::clone_repository(&config.git_repository, &target),
    )?;

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

    create_project_directories(&target, config.use_docker)?;

    let renderer = TemplateRenderer::new()?;
    renderer.render_project(config, &target)?;

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
