use clap::{CommandFactory, Parser};
use odoo_developer_kit::cli::Cli;

#[test]
fn clap_command_is_valid() {
    Cli::command().debug_assert();
}

#[test]
fn create_accepts_requested_flags_and_project_path_alias() {
    assert!(
        Cli::try_parse_from([
            "odk",
            "create",
            "--project-name",
            "burma",
            "--projec-path=/Users/agga/Documents/python-dev/odoo-dev",
            "--docker",
            "--pycharm",
            "--vscode",
            "--doctor",
        ])
        .is_ok()
    );
}
