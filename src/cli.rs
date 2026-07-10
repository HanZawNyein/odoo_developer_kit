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
    #[arg(long)]
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
                use_docker: args.docker.then_some(true),
                generate_pycharm: args.pycharm.then_some(true),
                generate_vscode: args.vscode.then_some(true),
            })
        }
        Commands::Upgrade => crate::upgrade::run(),
    }
}
