use crate::doctor::{CheckResult, CommandSpec, command_check};
use crate::utils::platform::Platform;

pub fn check(platform: Platform) -> CheckResult {
    command_check(
        "Python",
        &[
            CommandSpec {
                program: "python",
                args: &["--version"],
            },
            CommandSpec {
                program: "python3",
                args: &["--version"],
            },
        ],
        platform,
    )
}
