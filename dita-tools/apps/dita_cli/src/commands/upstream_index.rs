use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};
use clap::Args;
use dita_upstream::{Flavor, Source};

#[derive(Args)]
pub struct UpstreamIndexArgs {
    /// 索引写到哪里
    #[arg(long, default_value = "vocab/upstream-nodes.tsv")]
    pub out: PathBuf,

    /// 工具链目录；两个来源都在它下面（DITA-OT 与 oasis-dita 克隆）
    #[arg(long)]
    pub tools: Option<PathBuf>,

    /// 只打印，不写文件
    #[arg(long)]
    pub dry_run: bool,
}

/// 生成上游节点索引。
///
/// 版本号不在这里写死：DITA-OT 的目录名就是它的版本，OASIS 克隆的版本问 git。
/// `scripts/setup-env.sh` 是版本 SSOT，抄一份到 Rust 里就等于多了一处会漂的副本。
#[allow(clippy::print_stdout)]
pub fn run(args: &UpstreamIndexArgs) -> Result<()> {
    let tools = args.tools.clone().unwrap_or_else(default_tools_dir);
    let ot = find_dita_ot(&tools)?;
    let ot_version = ot
        .file_name()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_prefix("dita-ot-"))
        .unwrap_or("unknown")
        .to_string();
    let oasis = tools.join("oasis-dita");
    if !oasis.join("specification").is_dir() {
        bail!(
            "找不到 OASIS 规范源：{}（跑 just setup 装）",
            oasis.display()
        );
    }
    let oasis_version = git_version(&oasis);

    let sources = vec![
        Source {
            entry: ot.join("docsrc/site.ditamap"),
            root: ot.clone(),
            flavor: Flavor::DitaOt {
                version: ot_version.clone(),
            },
        },
        Source {
            // 规范正本的入口是 bookmap；archSpec-base.ditamap 那类分部 map 已与
            // 正本不同步（少了 accessibility 一章），照它走会漏节点
            entry: oasis.join("specification/dita-2.0-specification.ditamap"),
            root: oasis.clone(),
            flavor: Flavor::Oasis,
        },
    ];

    let index = dita_upstream::build_index(&sources)?;
    let versions = format!("DITA-OT {ot_version} · oasis-tcs/dita {oasis_version}");
    let tsv = dita_upstream::render_tsv(
        &index,
        &format!("dita-tools upstream-index --out {}", args.out.display()),
        &versions,
        &today(),
    );

    for note in &index.notes {
        println!("{note}");
    }
    println!("\n上游节点索引：{} 行（{versions}）", index.entries.len());

    if args.dry_run {
        print!("{tsv}");
        return Ok(());
    }
    if let Some(parent) = args.out.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(&args.out, tsv).with_context(|| format!("写不进 {}", args.out.display()))?;
    println!("→ {}", args.out.display());
    Ok(())
}

fn default_tools_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_default()
        .join("ws/tools")
}

/// 按目录名找 DITA-OT，不写死版本号：版本 SSOT 在 `scripts/setup-env.sh`。
fn find_dita_ot(tools: &Path) -> Result<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(tools)
        .with_context(|| format!("工具链目录读不了：{}", tools.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s.starts_with("dita-ot-"))
        })
        .collect();
    found.sort();
    match found.len() {
        0 => bail!("{} 下没有 dita-ot-*（跑 just setup 装）", tools.display()),
        1 => Ok(found.remove(0)),
        // 装了两个版本，索引该记哪个说不清——让人来定，别默默挑一个
        _ => bail!("{} 下有多个 DITA-OT：{found:?}", tools.display()),
    }
}

fn git_version(repo: &Path) -> String {
    let describe = git(repo, &["describe", "--tags", "--always"]);
    let sha = git(repo, &["rev-parse", "HEAD"]);
    match (describe, sha) {
        (Some(d), Some(s)) => format!("{d} ({s})"),
        (None, Some(s)) => s,
        _ => "unknown".to_string(),
    }
}

fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

/// 生成日期。没有 chrono 依赖，也不值得为一行日期加一个——
/// 拿系统的 date，取不到就留空而不是编一个。
fn today() -> String {
    Command::new("date")
        .arg("+%Y-%m-%d")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
