use odoo_developer_kit::create::template::TemplateRenderer;
use odoo_developer_kit::create::{
    ProjectConfig, ProjectOptions, database_name_from_project, validate_python_version,
};

fn sample_config() -> ProjectConfig {
    ProjectConfig::from_options(ProjectOptions {
        project_name: "geaai_odoo".to_owned(),
        git_repository: "git@github.com:company/template.git".to_owned(),
        odoo_version: "19.0".to_owned(),
        python_version: "3.13".to_owned(),
        postgres_version: "17".to_owned(),
        use_docker: true,
        generate_pycharm: true,
        generate_vscode: true,
    })
    .expect("sample config should be valid")
}

#[test]
fn validates_odoo_python_matrix() {
    assert!(validate_python_version("19.0", "3.13").is_ok());
    assert!(validate_python_version("19.0", "3.11").is_err());
    assert!(validate_python_version("18.0", "3.12").is_ok());
}

#[test]
fn normalizes_database_names() {
    assert_eq!(database_name_from_project("GEAAI Odoo"), "geaai_odoo");
    assert_eq!(database_name_from_project("  "), "odoo");
}

#[test]
fn renders_odoo_conf_template() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let rendered = renderer
        .render_to_string("odoo.conf.tera", &sample_config())
        .expect("template should render");

    assert!(rendered.contains("addons_path = addons,custom"));
    assert!(rendered.contains("db_host = db"));
}

#[test]
fn generated_pyproject_is_valid_toml() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let rendered = renderer
        .render_to_string("pyproject.toml.tera", &sample_config())
        .expect("template should render");
    let parsed: toml::Value = toml::from_str(&rendered).expect("generated TOML should parse");

    assert_eq!(parsed["project"]["name"].as_str(), Some("geaai_odoo"));
}
