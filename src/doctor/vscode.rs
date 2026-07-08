use crate::doctor::{CheckResult, CommandSpec, command_check};
use crate::utils::platform::Platform;

pub fn check(platform: Platform) -> CheckResult {
    command_check(
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
    )
}
