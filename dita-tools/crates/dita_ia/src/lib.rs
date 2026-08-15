mod orphan;
mod tree;

use anyhow::Result;
use dita_ast::DitaMap;
use dita_diagnostics::DiagnosticBag;
use dita_parser::parse_map;
use std::path::{Path, PathBuf};

pub use tree::count_topics;

pub struct IaReport {
    pub map: DitaMap,
    pub diagnostics: DiagnosticBag,
    pub orphans: Vec<PathBuf>,
    pub topics_root: PathBuf,
}

pub fn build_report(map_path: &Path, topics_root: &Path) -> Result<IaReport> {
    let (map, diagnostics) = parse_map(map_path)?;
    let orphans = orphan::find_orphans(&map, topics_root);
    Ok(IaReport {
        map,
        diagnostics,
        orphans,
        topics_root: topics_root.to_path_buf(),
    })
}

pub fn print_report(report: &IaReport) {
    println!("\n== 知识树（IA 视角）==\n");
    tree::print_tree(&report.map, &report.topics_root);

    if !report.orphans.is_empty() {
        println!(
            "\n⚠  孤儿 Topic（未被任何 Map 引用，共 {} 个）：",
            report.orphans.len()
        );
        for p in &report.orphans {
            let rel = p.strip_prefix(&report.topics_root).unwrap_or(p);
            println!("   topics/{}", rel.display());
        }
    } else {
        println!("\n✓  无孤儿 Topic");
    }

    let errs = report.diagnostics.error_count();
    let warns = report.diagnostics.warning_count();
    if errs > 0 || warns > 0 {
        println!("\n── 诊断 ──");
        for d in &report.diagnostics.items {
            let prefix = if d.is_error() { "❌" } else { "⚠ " };
            println!("  {prefix} {}: {}", d.path().display(), d.message());
        }
    }
}
