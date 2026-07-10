use odoo_developer_kit::create::template::TemplateRenderer;
use odoo_developer_kit::create::{
    ProjectConfig, ProjectOptions, database_name_from_project, validate_python_version,
};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn sample_config() -> ProjectConfig {
    ProjectConfig::from_options(ProjectOptions {
        project_name: "geaai_odoo".to_owned(),
        project_path: "/Users/agga/Documents/python-dev/odoo-dev/sample".to_owned(),
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
fn validates_supported_python_versions() {
    assert!(validate_python_version("3.8").is_ok());
    assert!(validate_python_version("3.10").is_ok());
    assert!(validate_python_version("3.13").is_ok());
    assert!(validate_python_version("3.7").is_err());
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

    assert_eq!(
        rendered,
        "[options]\naddons_path = /mnt/extra-addons\nadmin_passwd = admin\n"
    );
}

#[test]
fn renders_docker_compose_template() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let rendered = renderer
        .render_to_string("compose.yaml.tera", &sample_config())
        .expect("template should render");

    assert!(rendered.contains("web:"));
    assert!(rendered.contains("dockerfile: Dockerfile"));
    assert!(rendered.contains("./config:/etc/odoo"));
    assert!(!rendered.contains("./addons:/mnt/extra-addons"));
    assert!(rendered.contains("image: postgres:17"));
    assert!(rendered.contains("POSTGRES_PASSWORD_FILE=/run/secrets/postgresql_password"));
    assert!(rendered.contains("file: odoo_pg_pass"));
}

#[test]
fn render_project_does_not_create_env_files() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("odk-render-{unique}"));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    renderer
        .render_project(&sample_config(), &temp_dir)
        .expect("project should render");

    assert!(!temp_dir.join(".env").exists());
    assert!(!temp_dir.join(".env.example").exists());
}

#[test]
fn renders_dockerfile_template() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let rendered = renderer
        .render_to_string("Dockerfile.tera", &sample_config())
        .expect("template should render");

    assert!(rendered.contains("FROM odoo:19.0"));
    assert!(rendered.contains("COPY ./addons /mnt/extra-addons"));
}

#[test]
fn renders_gitignore_template() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let rendered = renderer
        .render_to_string("gitignore.tera", &sample_config())
        .expect("template should render");

    assert_eq!(
        rendered,
        ".DS_Store\n.idea/workspace.xml\n.odoo-data/\n.venv/\ntarget/\n__pycache__/\n*.py[cod]\n*.egg-info/\ndist/\nbuild/\n\n.idea/\n.vscode/\nconfig/\nodoo.conf\nodoo_pg_pass\n"
    );
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
