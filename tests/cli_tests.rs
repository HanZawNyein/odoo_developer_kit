use clap::CommandFactory;
use odoo_developer_kit::cli::Cli;

#[test]
fn clap_command_is_valid() {
    Cli::command().debug_assert();
}
