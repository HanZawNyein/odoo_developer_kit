use crate::doctor::{CheckResult, CommandSpec, command_check};
use crate::utils::platform::Platform;

pub fn check(platform: Platform) -> CheckResult {
    command_check(
        "Docker",
        &[CommandSpec {
            program: "docker",
            args: &["--version"],
        }],
        platform,
    )
}
