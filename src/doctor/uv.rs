use crate::doctor::{CheckResult, CommandSpec, command_check};
use crate::utils::platform::Platform;

pub fn check(platform: Platform) -> CheckResult {
    command_check(
        "uv",
        &[CommandSpec {
            program: "uv",
            args: &["--version"],
        }],
        platform,
    )
}
