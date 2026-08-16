use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct LintArgs {
    /// Topic files or directories to lint
    #[arg(default_values_os_t = vec![PathBuf::from("topics")])]
    pub paths: Vec<PathBuf>,

    /// Subject scheme supplying genre values and their metadata
    #[arg(long, default_value = "vocab/subjectScheme.ditamap")]
    pub vocab: PathBuf,
}

/// R12–R15 per-topic content rules. Severity follows maturity — drafts warn,
/// curated and verified error — so this is the promotion gate.
#[allow(clippy::print_stdout)]
pub fn run(args: &LintArgs) -> Result<()> {
    let (vocab, vdiag) = dita_vocab::parse_vocab(&args.vocab)?;
    for d in &vdiag.items {
        println!("{}: {}", d.path().display(), d.message());
    }

    let mut files = Vec::new();
    for p in &args.paths {
        if p.is_dir() {
            collect(p, &mut files);
        } else {
            files.push(p.clone());
        }
    }
    files.sort();

    let (mut errors, mut warnings) = (0usize, 0usize);
    for f in &files {
        let diag = dita_lint::lint_topic(f, &vocab)?;
        for d in &diag.items {
            let sev = if d.is_error() { "error" } else { "warning" };
            println!("{} [{sev}] {}", d.path().display(), d.message());
        }
        errors += diag.error_count();
        warnings += diag.warning_count();
    }
    println!(
        "\nlint：{} 个文件，{errors} error / {warnings} warning（draft 记 warning，晋级 curated 须清零）",
        files.len()
    );
    if errors > 0 {
        std::process::exit(1);
    }
    Ok(())
}

fn collect(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            collect(&p, out);
        } else if p.extension().and_then(|x| x.to_str()) == Some("dita") {
            out.push(p);
        }
    }
}
