use std::fs;
use std::path::{Path, PathBuf};

use tera::{Context, Tera};

use crate::create::ProjectConfig;
use crate::error::OdkError;

pub struct TemplateRenderer {
    tera: Tera,
}

impl TemplateRenderer {
    pub fn new() -> Result<Self, OdkError> {
        let mut tera = Tera::default();
        add_template(
            &mut tera,
            "compose.yaml.tera",
            include_str!("../../templates/compose.yaml.tera"),
        )?;
        add_template(
            &mut tera,
            "Dockerfile.tera",
            include_str!("../../templates/Dockerfile.tera"),
        )?;
        add_template(
            &mut tera,
            "odoo.conf.tera",
            include_str!("../../templates/odoo.conf.tera"),
        )?;
        add_template(
            &mut tera,
            "env.tera",
            include_str!("../../templates/env.tera"),
        )?;
        add_template(
            &mut tera,
            "gitignore.tera",
            include_str!("../../templates/gitignore.tera"),
        )?;
        add_template(
            &mut tera,
            "pyproject.toml.tera",
            include_str!("../../templates/pyproject.toml.tera"),
        )?;
        add_template(
            &mut tera,
            "README.md.tera",
            include_str!("../../templates/README.md.tera"),
        )?;
        add_template(
            &mut tera,
            "idea/misc.xml.tera",
            include_str!("../../templates/idea/misc.xml.tera"),
        )?;
        add_template(
            &mut tera,
            "idea/modules.xml.tera",
            include_str!("../../templates/idea/modules.xml.tera"),
        )?;
        add_template(
            &mut tera,
            "idea/runConfigurations/odoo.xml.tera",
            include_str!("../../templates/idea/runConfigurations/odoo.xml.tera"),
        )?;
        add_template(
            &mut tera,
            "vscode/settings.json.tera",
            include_str!("../../templates/vscode/settings.json.tera"),
        )?;
        add_template(
            &mut tera,
            "vscode/launch.json.tera",
            include_str!("../../templates/vscode/launch.json.tera"),
        )?;

        Ok(Self { tera })
    }

    pub fn render_to_string(
        &self,
        template_name: &str,
        config: &ProjectConfig,
    ) -> Result<String, OdkError> {
        let context = Context::from_serialize(config)?;
        self.tera
            .render(template_name, &context)
            .map_err(OdkError::Template)
    }

    pub fn render_project(&self, config: &ProjectConfig, target: &Path) -> Result<(), OdkError> {
        let mut files = vec![
            ("env.tera", PathBuf::from(".env")),
            ("gitignore.tera", PathBuf::from(".gitignore")),
            ("odoo.conf.tera", PathBuf::from("odoo.conf")),
            ("README.md.tera", PathBuf::from("README.md")),
            ("pyproject.toml.tera", PathBuf::from("pyproject.toml")),
        ];

        if config.use_docker {
            files.push(("compose.yaml.tera", PathBuf::from("compose.yaml")));
            files.push(("Dockerfile.tera", PathBuf::from("Dockerfile")));
        }

        if config.generate_pycharm {
            files.push(("idea/misc.xml.tera", PathBuf::from(".idea/misc.xml")));
            files.push(("idea/modules.xml.tera", PathBuf::from(".idea/modules.xml")));
            files.push((
                "idea/runConfigurations/odoo.xml.tera",
                PathBuf::from(".idea/runConfigurations/odoo.xml"),
            ));
        }

        if config.generate_vscode {
            files.push((
                "vscode/settings.json.tera",
                PathBuf::from(".vscode/settings.json"),
            ));
            files.push((
                "vscode/launch.json.tera",
                PathBuf::from(".vscode/launch.json"),
            ));
        }

        for (template_name, relative_path) in files {
            self.render_file(template_name, config, &target.join(relative_path))?;
        }

        Ok(())
    }

    fn render_file(
        &self,
        template_name: &str,
        config: &ProjectConfig,
        destination: &Path,
    ) -> Result<(), OdkError> {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let rendered = self.render_to_string(template_name, config)?;
        fs::write(destination, rendered)?;
        Ok(())
    }
}

fn add_template(tera: &mut Tera, name: &str, content: &str) -> Result<(), OdkError> {
    tera.add_raw_template(name, content)?;
    Ok(())
}
