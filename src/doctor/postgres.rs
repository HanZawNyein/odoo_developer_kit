use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::doctor::{CheckResult, CommandSpec, command_check, version_from_output};
use crate::utils::command::{run_command, string_args};
use crate::utils::platform::Platform;

pub fn check(platform: Platform) -> CheckResult {
    match platform {
        Platform::MacOs => check_macos(platform),
        _ => command_check("PostgreSQL", &postgres_command_specs(), platform),
    }
}

fn check_macos(platform: Platform) -> CheckResult {
    let command_result = command_check("PostgreSQL", &postgres_command_specs(), platform);

    if command_result.installed {
        return command_result;
    }

    match find_macos_postgres_version() {
        Some(version) => CheckResult::installed("PostgreSQL", Some(version)),
        None => CheckResult::missing("PostgreSQL", platform),
    }
}

fn postgres_command_specs() -> [CommandSpec<'static>; 4] {
    [
        CommandSpec {
            program: "psql",
            args: &["--version"],
        },
        CommandSpec {
            program: "postgres",
            args: &["--version"],
        },
        CommandSpec {
            program: "pg_ctl",
            args: &["--version"],
        },
        CommandSpec {
            program: "pg_config",
            args: &["--version"],
        },
    ]
}

fn find_macos_postgres_version() -> Option<String> {
    for path in macos_postgres_binary_paths() {
        if let Some(version) = postgres_tool_version(&path) {
            return Some(version);
        }
    }

    find_macos_postgres_app_version()
}

fn find_macos_postgres_app_version() -> Option<String> {
    for app_path in macos_postgres_app_paths() {
        if !app_path.exists() {
            continue;
        }

        return Some(
            postgres_app_version_from_dirs(&app_path).unwrap_or_else(|| "Unknown".to_owned()),
        );
    }
    None
}

fn macos_postgres_app_paths() -> Vec<PathBuf> {
    let mut paths = vec![PathBuf::from("/Applications/Postgres.app")];
    if let Some(home) = env::var_os("HOME") {
        paths.push(
            PathBuf::from(home)
                .join("Applications")
                .join("Postgres.app"),
        );
    }
    paths
}

fn macos_postgres_binary_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for root in ["/opt/homebrew", "/usr/local"] {
        for package in postgres_packages() {
            for binary in postgres_binaries() {
                paths.push(
                    PathBuf::from(root)
                        .join("opt")
                        .join(package)
                        .join("bin")
                        .join(binary),
                );
            }
        }
    }

    for version in postgres_versions() {
        for binary in postgres_binaries() {
            paths.push(
                PathBuf::from("/Library")
                    .join("PostgreSQL")
                    .join(version)
                    .join("bin")
                    .join(binary),
            );
        }
    }

    for app_path in macos_postgres_app_paths() {
        for version in ["latest"].into_iter().chain(postgres_versions()) {
            for binary in postgres_binaries() {
                paths.push(
                    app_path
                        .join("Contents")
                        .join("Versions")
                        .join(version)
                        .join("bin")
                        .join(binary),
                );
            }
        }
    }

    paths
}

fn postgres_tool_version(path: &Path) -> Option<String> {
    let program = path.to_string_lossy();
    let args = string_args(&["--version"]);
    let output = run_command(&program, &args, None).ok()?;
    if output.success() {
        version_from_output(&output.combined_output())
    } else {
        None
    }
}

fn postgres_binaries() -> [&'static str; 4] {
    ["psql", "postgres", "pg_ctl", "pg_config"]
}

fn postgres_packages() -> [&'static str; 7] {
    [
        "postgresql",
        "postgresql@18",
        "postgresql@17",
        "postgresql@16",
        "postgresql@15",
        "postgresql@14",
        "libpq",
    ]
}

fn postgres_versions() -> [&'static str; 6] {
    ["18", "17", "16", "15", "14", "13"]
}

fn postgres_app_version_from_dirs(app_path: &Path) -> Option<String> {
    let versions_dir = app_path.join("Contents").join("Versions");
    let entries = fs::read_dir(versions_dir).ok()?;

    let mut versions = entries
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name != "latest")
        .filter_map(|name| version_from_output(&name))
        .collect::<Vec<_>>();

    versions.sort();
    versions.pop()
}
