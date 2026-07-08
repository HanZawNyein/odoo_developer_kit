use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::doctor::{CheckResult, CommandSpec, command_check};
use crate::utils::platform::Platform;

pub fn check(platform: Platform) -> CheckResult {
    match platform {
        Platform::MacOs => check_macos(platform),
        _ => command_check(
            "VS Code",
            &[
                CommandSpec {
                    program: "code",
                    args: &["--version"],
                },
                CommandSpec {
                    program: "code.cmd",
                    args: &["--version"],
                },
            ],
            platform,
        ),
    }
}

fn check_macos(platform: Platform) -> CheckResult {
    let command_result = command_check(
        "VS Code",
        &[
            CommandSpec {
                program: "code",
                args: &["--version"],
            },
            CommandSpec {
                program: "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code",
                args: &["--version"],
            },
            CommandSpec {
                program: "code.cmd",
                args: &["--version"],
            },
        ],
        platform,
    );

    if command_result.installed {
        return command_result;
    }

    match find_macos_vscode_version() {
        Some(version) => CheckResult::installed("VS Code", Some(version)),
        None => CheckResult::missing("VS Code", platform),
    }
}

fn find_macos_vscode_version() -> Option<String> {
    for app_path in macos_vscode_app_paths() {
        if app_path.exists() {
            let info_plist = app_path.join("Contents").join("Info.plist");
            return Some(
                read_macos_bundle_version(&info_plist).unwrap_or_else(|| "Unknown".to_owned()),
            );
        }
    }
    None
}

fn macos_vscode_app_paths() -> Vec<PathBuf> {
    let mut paths = vec![PathBuf::from("/Applications/Visual Studio Code.app")];
    if let Some(home) = env::var_os("HOME") {
        paths.push(
            PathBuf::from(home)
                .join("Applications")
                .join("Visual Studio Code.app"),
        );
    }
    paths
}

fn read_macos_bundle_version(info_plist: &Path) -> Option<String> {
    let content = fs::read_to_string(info_plist).ok()?;
    let key_index = content.find("<key>CFBundleShortVersionString</key>")?;
    let after_key = content.get(key_index..)?;
    let start_tag = after_key.find("<string>")?;
    let version_start = key_index + start_tag + "<string>".len();
    let remaining = content.get(version_start..)?;
    let end_tag = remaining.find("</string>")?;
    remaining
        .get(..end_tag)
        .map(|value| value.trim().to_owned())
}
