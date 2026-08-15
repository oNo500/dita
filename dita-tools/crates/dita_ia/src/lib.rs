mod orphan;
mod tree;

use anyhow::Result;
use dita_ast::DitaMap;
use dita_diagnostics::DiagnosticBag;
use dita_parser::parse_map;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub use tree::count_topics;

pub struct IaReport {
    /// Maps rendered as trees, in the order they were requested.
    pub display: Vec<DitaMap>,
    /// Every map consulted when deciding orphanhood (display maps included).
    pub consulted: Vec<DitaMap>,
    pub diagnostics: DiagnosticBag,
    pub orphans: Vec<PathBuf>,
    pub topics_root: PathBuf,
}

/// Build the IA report.
///
/// `display_maps` are rendered as trees. `maps_dir`, when given, is scanned for
/// every `.ditamap` so that orphan detection accounts for deliverable and
/// glossary maps too, not just what hangs off the root.
pub fn build_report(
    display_maps: &[PathBuf],
    topics_root: &Path,
    maps_dir: Option<&Path>,
) -> Result<IaReport> {
    let mut diagnostics = DiagnosticBag::default();
    let mut display = Vec::new();
    let mut consulted = Vec::new();
    let mut seen = HashSet::new();

    for path in display_maps {
        let (map, diag) = parse_map(path)?;
        diagnostics.items.extend(diag.items);
        seen.insert(map.path.clone());
        consulted.push(map.clone());
        display.push(map);
    }

    if let Some(dir) = maps_dir {
        for path in orphan::find_maps(dir) {
            // canonicalize before the seen check: the same map reached by a
            // different spelling of its path must not be parsed twice
            let canonical = path.canonicalize().unwrap_or(path);
            if seen.contains(&canonical) {
                continue;
            }
            let (map, diag) = parse_map(&canonical)?;
            diagnostics.items.extend(diag.items);
            seen.insert(map.path.clone());
            consulted.push(map);
        }
    }

    let orphans = orphan::find_orphans(&consulted, topics_root);
    Ok(IaReport {
        display,
        consulted,
        diagnostics,
        orphans,
        topics_root: topics_root.canonicalize().unwrap_or_else(|_| topics_root.to_path_buf()),
    })
}

pub fn print_report(report: &IaReport) {
    println!("\n== 知识树（IA 视角）==");
    for map in &report.display {
        println!();
        tree::print_tree(map);
    }

    println!(
        "\n── 孤儿判定：参考了 {} 个 map ──",
        report.consulted.len()
    );
    if report.orphans.is_empty() {
        println!("✓  无孤儿 Topic");
    } else {
        println!("⚠  孤儿 Topic（未被任何 map 引用，共 {} 个）：", report.orphans.len());
        for p in &report.orphans {
            // both sides are canonical here, so the prefix actually strips
            let rel = p.strip_prefix(&report.topics_root).unwrap_or(p);
            println!("   {}", rel.display());
        }
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
