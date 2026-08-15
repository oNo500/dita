use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct IaArgs {
    /// Root .ditamap to render; repeat to render several
    #[arg(long, default_values_os_t = vec![PathBuf::from("maps/root.ditamap")])]
    pub map: Vec<PathBuf>,

    /// Path to the topics root directory
    #[arg(long, default_value = "topics")]
    pub topics: PathBuf,

    /// Directory scanned for every .ditamap when deciding orphanhood, so that
    /// topics reachable only from a deliverable map are not reported as orphans
    #[arg(long, default_value = "maps")]
    pub maps_dir: PathBuf,

    /// Judge orphanhood from --map alone, ignoring --maps-dir
    #[arg(long)]
    pub root_only: bool,

    /// Subject scheme supplying the controlled values. Missing file = value
    /// checks are skipped and said so, never guessed at.
    #[arg(long, default_value = "vocab/subjectScheme.ditamap")]
    pub vocab: PathBuf,

    /// Expand the per-branch tables behind the skeleton
    #[arg(long)]
    pub details: bool,

    /// Limit how many levels of the subject tree are expanded
    #[arg(long)]
    pub depth: Option<usize>,
}

pub fn run(args: &IaArgs) -> Result<()> {
    let maps_dir = if args.root_only {
        None
    } else {
        Some(args.maps_dir.as_path())
    };
    let report = dita_ia::build_report(&args.map, &args.topics, maps_dir, Some(&args.vocab))?;
    dita_ia::print_report(&report, args.details, args.depth);
    if report.diagnostics.has_errors() {
        std::process::exit(1);
    }
    Ok(())
}
