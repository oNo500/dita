use anyhow::Result;
use clap::{Args, ValueEnum};
use std::path::PathBuf;

/// 报告的两个面。
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Format {
    /// 终端里读的骨架树（默认）
    Human,
    /// 机器读的全量报告，字段契约见 `dita_ia` 的 `json` 模块
    Json,
}

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

    /// Output shape. `json` is the machine face: always the full report, since
    /// --details and --depth exist to fit a terminal, and a contract that
    /// changes shape with the command line is not one.
    #[arg(long, value_enum, default_value_t = Format::Human)]
    pub format: Format,

    /// Limit how many levels of the subject tree are expanded
    #[arg(long)]
    pub depth: Option<usize>,
}

#[allow(clippy::print_stdout)]
pub fn run(args: &IaArgs) -> Result<()> {
    let maps_dir = if args.root_only {
        None
    } else {
        Some(args.maps_dir.as_path())
    };
    let report = dita_ia::build_report(&args.map, &args.topics, maps_dir, Some(&args.vocab))?;
    match args.format {
        Format::Human => dita_ia::print_report(&report, args.details, args.depth),
        // pretty，不是紧凑一行：这份报告也会被人拿去 diff 两次运行的差异
        Format::Json => println!(
            "{}",
            serde_json::to_string_pretty(&dita_ia::json_report(&report))?
        ),
    }
    if report.diagnostics.has_errors() {
        std::process::exit(1);
    }
    Ok(())
}
