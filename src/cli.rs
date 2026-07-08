use anyhow::Result;
use clap::{Parser, Subcommand};

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
    Create,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Doctor => crate::doctor::run(),
        Commands::Create => crate::create::run(),
    }
}
