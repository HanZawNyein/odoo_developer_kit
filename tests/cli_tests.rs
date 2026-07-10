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
            "--psql=17",
            "--pycharm",
            "--vscode",
            "--doctor",
        ])
        .is_ok()
    );
}

#[test]
fn create_rejects_postgres_version_without_docker() {
    assert!(
        Cli::try_parse_from([
            "odk",
            "create",
            "--project-name",
            "burma",
            "--project-path=/Users/agga/Documents/python-dev/odoo-dev",
            "--postgres-version",
            "17",
        ])
        .is_err()
    );
    assert!(Cli::try_parse_from(["odk", "create", "--psql=17"]).is_err());
}
