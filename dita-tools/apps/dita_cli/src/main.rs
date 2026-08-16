mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dita-tools",
    about = "DITA authoring and analysis tools",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show IA overview: knowledge tree, orphan topics, and diagnostics
    Ia(commands::ia::IaArgs),
    /// Per-topic content rules R12–R15 (genre, structure, source labels, register)
    Lint(commands::lint::LintArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Ia(args) => commands::ia::run(&args),
        Commands::Lint(args) => commands::lint::run(&args),
    }
}
