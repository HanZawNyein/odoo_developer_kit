use std::cmp::Ordering;

use anyhow::Result;
use console::style;
use serde_json::Value;

use crate::error::OdkError;
use crate::utils::command::{run_command, run_required};
use crate::utils::platform::Platform;

const LATEST_RELEASE_API: &str =
    "https://api.github.com/repos/HanZawNyein/odoo_developer_kit/releases/latest";
const INSTALLER_SH: &str = "https://github.com/HanZawNyein/odoo_developer_kit/releases/latest/download/odoo-developer-kit-installer.sh";
const INSTALLER_PS1: &str = "https://github.com/HanZawNyein/odoo_developer_kit/releases/latest/download/odoo-developer-kit-installer.ps1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionStatus {
    Latest { current: String, latest: String },
    Outdated { current: String, latest: String },
    Unknown { current: String, reason: String },
}

pub fn run() -> Result<()> {
    println!("{}", style("Odoo Developer Kit Upgrade").bold());
    println!();
    println!("Current version: {}", current_version());

    match latest_version_status() {
        VersionStatus::Latest { latest, .. } => {
            println!("Latest version: {latest}");
            println!("{}", style("ODK is already up to date.").green().bold());
            Ok(())
        }
        VersionStatus::Outdated { current: _, latest } => {
            println!("Latest version: {latest}");
            run_installer(Platform::detect())?;
            println!("{}", style("Upgrade complete.").green().bold());
            Ok(())
        }
        VersionStatus::Unknown { reason, .. } => {
            println!(
                "{}",
                style("Could not check latest version.").yellow().bold()
            );
            println!("{reason}");
            println!("Running latest release installer anyway.");
            run_installer(Platform::detect())?;
            println!("{}", style("Upgrade complete.").green().bold());
            Ok(())
        }
    }
}

pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn latest_version_status() -> VersionStatus {
    let current = current_version().to_owned();
    match latest_release_version() {
        Ok(latest) if is_newer_version(&latest, &current) => {
            VersionStatus::Outdated { current, latest }
        }
        Ok(latest) => VersionStatus::Latest { current, latest },
        Err(error) => VersionStatus::Unknown {
            current,
            reason: error.to_string(),
        },
    }
}

fn latest_release_version() -> Result<String, OdkError> {
    let args = vec![
        "-fsSL".to_owned(),
        "--connect-timeout".to_owned(),
        "2".to_owned(),
        "--max-time".to_owned(),
        "5".to_owned(),
        "-H".to_owned(),
        "User-Agent: odk".to_owned(),
        LATEST_RELEASE_API.to_owned(),
    ];
    let output = run_command("curl", &args, None)?;
    if !output.success() {
        return Err(OdkError::CommandFailed {
            command: output.command_line(),
            status: output
                .status
                .map_or_else(|| "unknown".to_owned(), |status| status.to_string()),
            stderr: output.combined_output(),
        });
    }

    parse_latest_release_version(&output.stdout)
        .ok_or_else(|| OdkError::InvalidReleaseResponse("missing release tag_name".to_owned()))
}

pub fn parse_latest_release_version(json: &str) -> Option<String> {
    let value: Value = serde_json::from_str(json).ok()?;
    normalize_version(value.get("tag_name")?.as_str()?)
}

pub fn normalize_version(version: &str) -> Option<String> {
    let normalized = version
        .trim()
        .trim_start_matches('v')
        .trim_start_matches('V');
    if normalized.is_empty() {
        None
    } else {
        Some(normalized.to_owned())
    }
}

pub fn is_newer_version(latest: &str, current: &str) -> bool {
    compare_versions(latest, current) == Ordering::Greater
}

fn compare_versions(left: &str, right: &str) -> Ordering {
    let left_parts = version_parts(left);
    let right_parts = version_parts(right);
    let max_len = left_parts.len().max(right_parts.len());

    for index in 0..max_len {
        let left_part = left_parts.get(index).copied().unwrap_or(0);
        let right_part = right_parts.get(index).copied().unwrap_or(0);
        match left_part.cmp(&right_part) {
            Ordering::Equal => continue,
            ordering => return ordering,
        }
    }

    Ordering::Equal
}

fn version_parts(version: &str) -> Vec<u64> {
    version
        .split(|character: char| !character.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .filter_map(|part| part.parse::<u64>().ok())
        .collect()
}

fn run_installer(platform: Platform) -> Result<(), OdkError> {
    match platform {
        Platform::MacOs | Platform::Linux => {
            let args = vec![
                "-c".to_owned(),
                format!("curl --proto '=https' --tlsv1.2 -LsSf {INSTALLER_SH} | sh"),
            ];
            run_required("sh", &args, None)
        }
        Platform::Windows => {
            let args = vec![
                "-ExecutionPolicy".to_owned(),
                "Bypass".to_owned(),
                "-c".to_owned(),
                format!("irm {INSTALLER_PS1} | iex"),
            ];
            run_required("powershell", &args, None)
        }
        Platform::Unknown => Err(OdkError::UnsupportedPlatform(platform.name().to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use super::{is_newer_version, normalize_version, parse_latest_release_version};

    #[test]
    fn parses_latest_release_version() {
        let json = r#"{"tag_name":"v0.2.0"}"#;
        assert_eq!(parse_latest_release_version(json), Some("0.2.0".to_owned()));
    }

    #[test]
    fn normalizes_release_versions() {
        assert_eq!(normalize_version("v1.2.3"), Some("1.2.3".to_owned()));
        assert_eq!(normalize_version(" V1.2.3 "), Some("1.2.3".to_owned()));
        assert_eq!(normalize_version(""), None);
    }

    #[test]
    fn compares_release_versions() {
        assert!(is_newer_version("0.2.0", "0.1.9"));
        assert!(is_newer_version("1.0.0", "0.9.9"));
        assert!(!is_newer_version("0.1.1", "0.1.1"));
        assert!(!is_newer_version("0.1.0", "0.1.1"));
    }
}
