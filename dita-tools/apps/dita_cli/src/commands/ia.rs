use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct IaArgs {
    /// Path to the root .ditamap file
    #[arg(long, default_value = "maps/root.ditamap")]
    pub map: PathBuf,

    /// Path to the topics root directory
    #[arg(long, default_value = "topics")]
    pub topics: PathBuf,
}

pub fn run(args: IaArgs) -> Result<()> {
    let report = dita_ia::build_report(&args.map, &args.topics)?;
    dita_ia::print_report(&report);
    if report.diagnostics.has_errors() {
        std::process::exit(1);
    }
    Ok(())
}
