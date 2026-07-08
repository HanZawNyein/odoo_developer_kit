use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::doctor::{CheckResult, CommandSpec, command_check};
use crate::utils::command::{run_command, string_args};
use crate::utils::platform::Platform;

pub fn check(platform: Platform) -> CheckResult {
    match platform {
        Platform::MacOs => check_macos(platform),
        Platform::Linux => check_linux(platform),
        Platform::Windows => check_windows(platform),
        Platform::Unknown => command_check(
            "PyCharm",
            &[
                CommandSpec {
                    program: "pycharm",
                    args: &["--version"],
                },
                CommandSpec {
                    program: "charm",
                    args: &["--version"],
                },
            ],
            platform,
        ),
    }
}

fn check_macos(platform: Platform) -> CheckResult {
    let command_result = command_check(
        "PyCharm",
        &[
            CommandSpec {
                program: "pycharm",
                args: &["--version"],
            },
            CommandSpec {
                program: "charm",
                args: &["--version"],
            },
        ],
        platform,
    );

    if command_result.installed {
        return command_result;
    }

    match find_macos_pycharm_version() {
        Some(version) => CheckResult::installed("PyCharm", Some(version)),
        None => CheckResult::missing("PyCharm", platform),
    }
}

fn check_linux(platform: Platform) -> CheckResult {
    let command_result = command_check(
        "PyCharm",
        &[CommandSpec {
            program: "pycharm",
            args: &["--version"],
        }],
        platform,
    );

    if command_result.installed {
        return command_result;
    }

    let args = string_args(&["pycharm"]);
    match run_command("which", &args, None) {
        Ok(output) if output.success() => CheckResult::installed("PyCharm", None),
        _ => CheckResult::missing("PyCharm", platform),
    }
}

fn check_windows(platform: Platform) -> CheckResult {
    let command_result = command_check(
        "PyCharm",
        &[CommandSpec {
            program: "pycharm64.exe",
            args: &["--version"],
        }],
        platform,
    );

    if command_result.installed {
        return command_result;
    }

    match find_windows_pycharm_version() {
        Some(version) => CheckResult::installed("PyCharm", Some(version)),
        None => CheckResult::missing("PyCharm", platform),
    }
}

fn find_macos_pycharm_version() -> Option<String> {
    let mut roots = vec![PathBuf::from("/Applications")];
    if let Some(home_applications) = home_applications_dir() {
        roots.push(home_applications);
    }

    for root in roots {
        let entries = match fs::read_dir(root) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("PyCharm") && name.ends_with(".app") {
                let info_plist = entry.path().join("Contents").join("Info.plist");
                return Some(
                    read_macos_bundle_version(&info_plist).unwrap_or_else(|| "Unknown".to_owned()),
                );
            }
        }
    }
    None
}

fn home_applications_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(|home| PathBuf::from(home).join("Applications"))
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

fn find_windows_pycharm_version() -> Option<String> {
    for root in windows_program_files_roots() {
        let jetbrains = root.join("JetBrains");
        let entries = match fs::read_dir(jetbrains) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("PyCharm") {
                return Some(
                    read_product_info_version(&entry.path().join("product-info.json"))
                        .unwrap_or_else(|| "Unknown".to_owned()),
                );
            }
        }
    }
    None
}

fn windows_program_files_roots() -> Vec<PathBuf> {
    ["ProgramFiles", "ProgramFiles(x86)"]
        .iter()
        .filter_map(env::var_os)
        .map(PathBuf::from)
        .collect()
}

#[derive(Debug, Deserialize)]
struct ProductInfo {
    version: Option<String>,
}

fn read_product_info_version(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let product_info: ProductInfo = serde_json::from_str(&content).ok()?;
    product_info.version
}
