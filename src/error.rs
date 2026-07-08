use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OdkError {
    #[error("command not found: {0}")]
    CommandNotFound(String),

    #[error("command `{command}` failed with status {status}: {stderr}")]
    CommandFailed {
        command: String,
        status: String,
        stderr: String,
    },

    #[error(
        "invalid Python version `{python_version}` for Odoo `{odoo_version}`; supported versions: {supported}"
    )]
    InvalidPythonVersion {
        odoo_version: String,
        python_version: String,
        supported: String,
    },

    #[error("target directory already exists and is not empty: {0}")]
    NonEmptyTarget(PathBuf),

    #[error("unsupported operating system or architecture: {0}")]
    UnsupportedPlatform(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Template(#[from] tera::Error),
}
