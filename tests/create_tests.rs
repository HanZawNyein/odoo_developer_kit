use odoo_developer_kit::create::template::TemplateRenderer;
use odoo_developer_kit::create::{
    ProjectConfig, ProjectOptions, database_name_from_project, project_path_from_input,
    validate_python_version,
};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn sample_config() -> ProjectConfig {
    ProjectConfig::from_options(ProjectOptions {
        project_name: "geaai_odoo".to_owned(),
        project_path: "/Users/agga/Documents/python-dev/odoo-dev/sample".to_owned(),
        git_repository: "git@github.com:company/template.git".to_owned(),
        odoo_source_path: "/Users/agga/src/odoo".to_owned(),
        odoo_version: "19.0".to_owned(),
        python_version: "3.13".to_owned(),
        postgres_version: "17".to_owned(),
        use_docker: true,
        generate_pycharm: true,
        generate_vscode: true,
    })
    .expect("sample config should be valid")
}

fn sample_local_config() -> ProjectConfig {
    ProjectConfig::from_options(ProjectOptions {
        project_name: "odoo-dev".to_owned(),
        project_path: "/Users/agga/Documents/python-dev".to_owned(),
        git_repository: "git@github.com:company/template.git".to_owned(),
        odoo_source_path: "/Users/agga/Documents/src/odoo19c".to_owned(),
        odoo_version: "19.0".to_owned(),
        python_version: "3.13".to_owned(),
        postgres_version: "17".to_owned(),
        use_docker: false,
        generate_pycharm: true,
        generate_vscode: true,
    })
    .expect("sample local config should be valid")
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
fn appends_project_name_to_parent_path() {
    assert_eq!(
        project_path_from_input("/Users/agga/Documents/python-dev/odoo-dev", "burma"),
        "/Users/agga/Documents/python-dev/odoo-dev/burma"
    );
}

#[test]
fn renders_odoo_conf_template() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let rendered = renderer
        .render_to_string("odoo.conf.tera", &sample_config())
        .expect("template should render");

    assert_eq!(
        rendered,
        "[options]\naddons_path = /Users/agga/src/odoo/addons,/Users/agga/Documents/python-dev/odoo-dev/sample/geaai_odoo/addons\nadmin_passwd = admin\ndb_user=odoo\ndb_password=odoo\n"
    );
}

#[test]
fn renders_local_odoo_conf_template() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let rendered = renderer
        .render_to_string("odoo.conf.tera", &sample_local_config())
        .expect("template should render");

    assert_eq!(
        rendered,
        "[options]\naddons_path = /Users/agga/Documents/src/odoo19c/addons,/Users/agga/Documents/python-dev/odoo-dev/addons\nadmin_passwd = admin\ndb_user=odoo\ndb_password=odoo\n"
    );
}

#[test]
fn renders_docker_odoo_conf_template() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let rendered = renderer
        .render_to_string("odoo.docker.conf.tera", &sample_config())
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
fn render_project_with_docker_creates_both_odoo_conf_files() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("odk-docker-render-{unique}"));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    renderer
        .render_project(&sample_config(), &temp_dir)
        .expect("project should render");

    assert!(temp_dir.join("config/odoo.conf").exists());
    assert!(temp_dir.join("odoo.conf").exists());

    assert_eq!(
        fs::read_to_string(temp_dir.join("config/odoo.conf"))
            .expect("docker config should be readable"),
        "[options]\naddons_path = /mnt/extra-addons\nadmin_passwd = admin\n"
    );
    assert_eq!(
        fs::read_to_string(temp_dir.join("odoo.conf")).expect("local config should be readable"),
        "[options]\naddons_path = /Users/agga/src/odoo/addons,/Users/agga/Documents/python-dev/odoo-dev/sample/geaai_odoo/addons\nadmin_passwd = admin\ndb_user=odoo\ndb_password=odoo\n"
    );
}

#[test]
fn render_project_generates_reference_pycharm_layout() {
    let renderer = TemplateRenderer::new().expect("templates should load");
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("odk-pycharm-render-{unique}"));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    renderer
        .render_project(&sample_local_config(), &temp_dir)
        .expect("project should render");

    for relative_path in [
        ".idea/.gitignore",
        ".idea/odoo-dev.iml",
        ".idea/inspectionProfiles/profiles_settings.xml",
        ".idea/misc.xml",
        ".idea/modules.xml",
        ".idea/runConfigurations/odoo.xml",
        ".idea/vcs.xml",
    ] {
        assert!(
            temp_dir.join(relative_path).is_file(),
            "expected generated PyCharm file: {relative_path}"
        );
    }
    assert!(!temp_dir.join(".idea/workspace.xml").exists());

    let module = fs::read_to_string(temp_dir.join(".idea/odoo-dev.iml"))
        .expect("module file should be readable");
    assert!(module.contains("jdkName=\"uv (odoo-dev)\""));
    assert!(module.contains("file:///Users/agga/Documents/src/odoo19c"));

    let modules = fs::read_to_string(temp_dir.join(".idea/modules.xml"))
        .expect("modules file should be readable");
    assert!(modules.contains("$PROJECT_DIR$/.idea/odoo-dev.iml"));

    let run_configuration = fs::read_to_string(temp_dir.join(".idea/runConfigurations/odoo.xml"))
        .expect("run configuration should be readable");
    assert!(run_configuration.contains("type=\"Odoo\""));
    assert!(run_configuration.contains("/Users/agga/Documents/src/odoo19c/odoo-bin"));
    assert!(run_configuration.contains("-c $PROJECT_DIR$/odoo.conf"));
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
