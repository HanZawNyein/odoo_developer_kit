use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "odk")]
#[command(version)]
#[command(about = "Odoo Developer Kit")]
#[command(long_about = "A Flutter-like developer toolkit for Odoo developers.")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Check the local Odoo developer environment.
    Doctor,
    /// Create a new Odoo development project.
    Create(CreateArgs),
    /// Upgrade ODK to the latest GitHub release.
    Upgrade,
}

#[derive(Debug, Args)]
struct CreateArgs {
    /// Project/package name.
    #[arg(long)]
    project_name: Option<String>,
    /// Parent directory or complete project path.
    #[arg(long, alias = "projec-path")]
    project_path: Option<String>,
    /// Optional Git repository to clone before setup.
    #[arg(long)]
    git_repository: Option<String>,
    /// Local Odoo source code path.
    #[arg(long)]
    odoo_source_path: Option<String>,
    /// Odoo version, detected from source path when omitted.
    #[arg(long)]
    odoo_version: Option<String>,
    /// Python version to install with uv.
    #[arg(long)]
    python_version: Option<String>,
    /// PostgreSQL Docker image version.
    #[arg(long, alias = "psql", requires = "docker")]
    postgres_version: Option<String>,
    /// Generate Docker files.
    #[arg(long)]
    docker: bool,
    /// Generate PyCharm configuration.
    #[arg(long)]
    pycharm: bool,
    /// Generate VS Code configuration.
    #[arg(long)]
    vscode: bool,
    /// Run doctor before creating the project.
    #[arg(long)]
    doctor: bool,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Doctor => crate::doctor::run(),
        Commands::Create(args) => {
            let has_create_options = args.has_create_options();
            if args.doctor {
                crate::doctor::run()?;
                println!();
            }
            crate::create::run_with_options(crate::create::CreateCommandOptions {
                project_name: args.project_name,
                project_path: args.project_path,
                git_repository: args.git_repository,
                odoo_source_path: args.odoo_source_path,
                odoo_version: args.odoo_version,
                python_version: args.python_version,
                postgres_version: args.postgres_version,
                use_docker: create_flag_value(has_create_options, args.docker),
                generate_pycharm: create_flag_value(has_create_options, args.pycharm),
                generate_vscode: create_flag_value(has_create_options, args.vscode),
            })
        }
        Commands::Upgrade => crate::upgrade::run(),
    }
}

impl CreateArgs {
    fn has_create_options(&self) -> bool {
        self.project_name.is_some()
            || self.project_path.is_some()
            || self.git_repository.is_some()
            || self.odoo_source_path.is_some()
            || self.odoo_version.is_some()
            || self.python_version.is_some()
            || self.postgres_version.is_some()
            || self.docker
            || self.pycharm
            || self.vscode
    }
}

fn create_flag_value(has_create_options: bool, enabled: bool) -> Option<bool> {
    if has_create_options {
        Some(enabled)
    } else {
        enabled.then_some(true)
    }
}
