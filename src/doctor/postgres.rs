use crate::doctor::{CheckResult, CommandSpec, command_check};
use crate::utils::platform::Platform;

pub fn check(platform: Platform) -> CheckResult {
    command_check(
        "PostgreSQL",
        &[CommandSpec {
            program: "psql",
            args: &["--version"],
        }],
        platform,
    )
}
