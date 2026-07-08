use odoo_developer_kit::doctor::version_from_output;
use odoo_developer_kit::utils::command::first_version;

#[test]
fn parses_versions_from_command_output() {
    assert_eq!(first_version("uv 0.8.4"), Some("0.8.4".to_owned()));
    assert_eq!(
        version_from_output("psql (PostgreSQL) 17.5"),
        Some("17.5".to_owned())
    );
}
