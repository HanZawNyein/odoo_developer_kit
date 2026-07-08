use std::path::Path;

use crate::error::OdkError;
use crate::utils::command::run_required;

pub fn clone_repository(repository: &str, target: &Path) -> Result<(), OdkError> {
    let args = vec![
        "clone".to_owned(),
        repository.to_owned(),
        target.display().to_string(),
    ];
    run_required("git", &args, None)
}
