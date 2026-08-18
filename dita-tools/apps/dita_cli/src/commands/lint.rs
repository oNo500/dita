use anyhow::Result;
use clap::Args;
use dita_upstream::NodeIndex;
use std::path::PathBuf;

/// Exit code for "a check did not run". Mirrors `kb/scripts/review.sh`:
/// 1 = 有 error（确定的失败），2 = 有检查未执行（结果不能当通过依据）。
const EXIT_SKIPPED: i32 = 2;

#[derive(Args)]
pub struct LintArgs {
    /// Topic files or directories to lint
    #[arg(default_values_os_t = vec![PathBuf::from("topics")])]
    pub paths: Vec<PathBuf>,

    /// Subject scheme supplying genre values and their metadata
    #[arg(long, default_value = "vocab/subjectScheme.ditamap")]
    pub vocab: PathBuf,

    /// Upstream node index that R19 resolves declarations against
    #[arg(long, default_value = "vocab/upstream-nodes.tsv")]
    pub upstream_index: PathBuf,
}

/// R12–R16, R18 and R19 per-topic content rules. Severity follows maturity —
/// drafts warn, curated and verified error — so this is the promotion gate.
/// R18 is the exception (always an error); R19 follows the grading.
#[allow(clippy::print_stdout, clippy::print_stderr)]
pub fn run(args: &LintArgs) -> Result<()> {
    let (vocab, vdiag) = dita_vocab::parse_vocab(&args.vocab)?;
    for d in &vdiag.items {
        println!("{}: {}", d.path().display(), d.message());
    }

    // 索引缺失或读不了 → R19 走「未执行」通道。不是通过：把索引的故障当成
    // 「所有声明都解析不到」会报出一屏假错，当成「都解析得到」则是假绿，
    // 两者都比明说"这一层没跑"更坏（同 review.sh 第 4–5 行的纪律）。
    let upstream = match NodeIndex::load(&args.upstream_index) {
        Ok(index) => Some(index),
        Err(e) => {
            eprintln!("{e:#}");
            eprintln!(
                "R19（上游节点声明）未执行：读不到索引 {}（重新生成：just upstream-index）",
                args.upstream_index.display()
            );
            None
        }
    };

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
        let diag = dita_lint::lint_topic(f, &vocab, upstream.as_ref())?;
        for d in &diag.items {
            let sev = if d.is_error() { "error" } else { "warning" };
            println!("{} [{sev}] {}", d.path().display(), d.message());
        }
        errors += diag.error_count();
        warnings += diag.warning_count();
    }
    match &upstream {
        Some(index) => println!(
            "\n上游节点索引：{} 个节点（{}）",
            index.len(),
            index.provenance()
        ),
        None => println!("\n⚠️  R19 未执行（索引读不到），本次结果不含上游声明这一层"),
    }
    println!(
        "lint：{} 个文件，{errors} error / {warnings} warning（draft 记 warning，晋级 curated 须清零）",
        files.len()
    );
    // 失败比跳过确定，退出码取更确定的那个
    if errors > 0 {
        std::process::exit(1);
    }
    if upstream.is_none() {
        std::process::exit(EXIT_SKIPPED);
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
