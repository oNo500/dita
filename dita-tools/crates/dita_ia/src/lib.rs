mod consistency;
mod domain;
mod governance;
mod orphan;
mod paint;
mod render;
mod skeleton;
mod stats;
mod tree;

use anyhow::Result;
use dita_ast::{DitaMap, TopicMeta};
use dita_diagnostics::{Diagnostic, DiagnosticBag};
use dita_parser::{parse_map, parse_topic};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub use consistency::check_group_titles;
pub use domain::{Branches, branches};
pub use governance::{BenchmarkEntry, BranchPlan, ValueUsage};
pub use paint::Paint;
pub use render::print_report;
pub use skeleton::{Node, State};
pub use stats::{BranchStats, DomainCoverage};
pub use tree::count_topics;

pub struct IaReport {
    /// Maps rendered as trees, in the order they were requested.
    pub display: Vec<DitaMap>,
    /// Every map consulted when deciding orphanhood (display maps included).
    pub consulted: Vec<DitaMap>,
    pub topics: Vec<TopicMeta>,
    pub branch_stats: Vec<BranchStats>,
    pub coverage: Vec<DomainCoverage>,
    pub diagnostics: DiagnosticBag,
    pub orphans: Vec<PathBuf>,
    pub topics_root: PathBuf,
    /// False when no subject scheme was available, so value checks were skipped.
    pub vocab_loaded: bool,
    /// What the taxonomy plans per branch versus what the maps hold.
    pub plans: Vec<BranchPlan>,
    /// The benchmark registry — the taxonomy's own decay clock.
    pub benchmarks: Vec<BenchmarkEntry>,
    /// Which controlled values the content actually uses.
    pub value_usage: Vec<ValueUsage>,
    /// The subject tree with content hung on it — the view itself.
    pub skeleton: Vec<Node>,
    /// R17's reverse report: leaf subject keys the scheme registers that no
    /// topic names as its domain — the tree's empty leaves, sorted by key.
    pub empty_subject_leaves: Vec<String>,
}

/// Build the IA report.
///
/// `display_maps` are rendered as trees. `maps_dir`, when given, is scanned for
/// every `.ditamap` so that orphan detection accounts for deliverable and
/// glossary maps too, not just what hangs off the root. `vocab` supplies the
/// controlled values; without it the value checks are skipped rather than
/// guessed at.
///
/// # Errors
///
/// Returns `Err` when a map or topic cannot be read or parsed at all.
pub fn build_report(
    display_maps: &[PathBuf],
    topics_root: &Path,
    maps_dir: Option<&Path>,
    vocab: Option<&Path>,
) -> Result<IaReport> {
    let mut diagnostics = DiagnosticBag::default();
    let mut display = Vec::new();
    let mut consulted = Vec::new();
    let mut seen = HashSet::new();

    for path in display_maps {
        let (map, diag) = parse_map(path)?;
        diagnostics.items.extend(diag.items);
        consistency::check_group_titles(&map, &mut diagnostics);
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

    // every topic under topics_root, referenced or not: an orphan's metadata is
    // as interesting as any other, and often more so
    let mut topics = Vec::new();
    for path in orphan::find_topics(topics_root) {
        let (meta, diag) = parse_topic(&path)?;
        diagnostics.items.extend(diag.items);
        topics.push(meta);
    }

    let branch_map = display.first().map_or_else(Branches::default, branches);
    let branch_stats = stats::branch_stats(&branch_map, &topics);
    // subject key → its descendants, so coverage can roll up the taxonomy
    let descendants = vocab
        .filter(|p| p.exists())
        .and_then(|p| dita_vocab::parse_vocab(p).ok())
        .map_or_else(Default::default, |(v, _)| {
            let mut out: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> =
                std::collections::BTreeMap::new();
            if let Some(subject) = v.subject("subject") {
                collect_descendants(subject, &mut out);
            }
            out
        });
    let coverage = stats::domain_coverage(&branch_map, &topics, &descendants);

    let Governance {
        plans,
        benchmarks,
        value_usage,
        vocab_loaded,
    } = read_governance(vocab, &topics, &branch_map, &branch_stats, &mut diagnostics)?;

    let mut skeleton = Vec::new();
    let mut empty_subject_leaves = Vec::new();
    if let Some(path) = vocab.filter(|p| p.exists()) {
        let (vocabulary, _) = dita_vocab::parse_vocab(path)?;
        empty_subject_leaves = empty_leaves(&vocabulary, &topics);
        let benchmarks: std::collections::BTreeMap<String, String> =
            governance::benchmarks(&vocabulary)
                .iter()
                .filter_map(|b| {
                    let key = b.key.strip_prefix("bm-")?.to_string();
                    let date = b.last_benchmarked.clone()?;
                    Some((
                        key,
                        match b.due_months() {
                            Some(m) => format!("对标 {date}+{m}mo"),
                            None => format!("对标 {date}·触发"),
                        },
                    ))
                })
                .collect();
        skeleton = skeleton::build(&skeleton::Input {
            vocab: &vocabulary,
            topics: &topics,
            branches: &branch_map,
            coverage: &coverage,
            benchmarks: &benchmarks,
        });
    }

    check_domains(&coverage, &mut diagnostics);

    Ok(IaReport {
        display,
        consulted,
        topics,
        branch_stats,
        coverage,
        diagnostics,
        orphans,
        topics_root: topics_root
            .canonicalize()
            .unwrap_or_else(|_| topics_root.to_path_buf()),
        vocab_loaded,
        plans,
        benchmarks,
        value_usage,
        skeleton,
        empty_subject_leaves,
    })
}

/// What the vocabulary contributes to the report: the "ought" and the clock.
#[derive(Default)]
struct Governance {
    plans: Vec<BranchPlan>,
    benchmarks: Vec<BenchmarkEntry>,
    value_usage: Vec<ValueUsage>,
    vocab_loaded: bool,
}

/// Read the vocabulary-derived halves of governance. Absent vocabulary is not
/// an error — the report degrades to what the maps alone can say, and says so.
fn read_governance(
    vocab: Option<&Path>,
    topics: &[TopicMeta],
    branch_map: &Branches,
    branch_stats: &[BranchStats],
    diagnostics: &mut DiagnosticBag,
) -> Result<Governance> {
    let Some(path) = vocab.filter(|p| p.exists()) else {
        return Ok(Governance::default());
    };
    let (vocabulary, diag) = dita_vocab::parse_vocab(path)?;
    diagnostics.items.extend(diag.items);
    check_values(&vocabulary, topics, diagnostics);
    Ok(Governance {
        plans: governance::branch_plans(&vocabulary, branch_map, branch_stats),
        benchmarks: governance::benchmarks(&vocabulary),
        value_usage: governance::value_usage(&vocabulary, topics),
        vocab_loaded: true,
    })
}

/// Check tagged values against the subject scheme — the vocabulary is the only
/// source of legal values, so this is the one place that knows them.
/// Record每个 subject 键的全部后代键。
fn collect_descendants(
    subject: &dita_vocab::Subject,
    out: &mut std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
) {
    let mut kids = std::collections::BTreeSet::new();
    for child in &subject.children {
        kids.extend(child.all_keys());
        collect_descendants(child, out);
    }
    out.insert(subject.keys.clone(), kids);
}

fn check_values(vocab: &dita_vocab::Vocabulary, topics: &[TopicMeta], diag: &mut DiagnosticBag) {
    // R17: domain must name a subject key — it is the only link from a topic
    // to the taxonomy, and a typo silently detaches the topic from the
    // skeleton. enumerationdef cannot bind to a `data` element, so this check
    // has nowhere to live but here (see kb/schema/rules.sch R17).
    let subject_keys = vocab
        .subject("subject")
        .map(dita_vocab::Subject::all_keys)
        .unwrap_or_default();
    for meta in topics {
        if let Some(domain) = &meta.domain {
            if !subject_keys.is_empty() && !subject_keys.contains(domain) {
                diag.push(Diagnostic::error(
                    &meta.path,
                    format!(
                        "domain 值 \"{domain}\" 不是词表已注册的 subject key（R17）——请在 subjectScheme 注册该键，或改用已注册值"
                    ),
                ));
            }
        }
        for (attr, values) in [("dimension", &meta.dimensions), ("tool", &meta.tools)] {
            if let Some(legal) = vocab.legal_values(attr) {
                for value in values {
                    if !legal.contains(value) {
                        diag.push(Diagnostic::error(
                            &meta.path,
                            format!("@{attr} 值 \"{value}\" 不在词表中"),
                        ));
                    }
                }
            }
        }
        for (attr, value) in [
            ("maturity", meta.maturity.as_ref()),
            ("volatility", meta.volatility.as_ref()),
        ] {
            let (Some(value), Some(legal)) = (value, vocab.legal_values(attr)) else {
                continue;
            };
            if !legal.contains(value) {
                diag.push(Diagnostic::error(
                    &meta.path,
                    format!("@{attr} 值 \"{value}\" 不在词表中"),
                ));
            }
        }
    }
}

/// R17's reverse report: leaf subject keys the scheme registers but no topic
/// has claimed as its domain — the taxonomy's empty leaves.
///
/// Only leaves, not every empty node: an interior key such as `writing` is
/// empty exactly when every one of its children is, so listing it too would
/// just repeat the same gap under two names. The leaves are where a planner
/// decides whether to write, retire, or fold a key.
fn empty_leaves(vocab: &dita_vocab::Vocabulary, topics: &[TopicMeta]) -> Vec<String> {
    let Some(subject) = vocab.subject("subject") else {
        return Vec::new();
    };
    let claimed: HashSet<&str> = topics.iter().filter_map(|t| t.domain.as_deref()).collect();
    let mut out: Vec<String> = subject
        .leaf_keys()
        .into_iter()
        .filter(|key| !claimed.contains(key.as_str()))
        .collect();
    out.sort();
    out
}

/// A technology domain spread across several branches usually means a topic is
/// hanging in the wrong map, or a `domain` tag has gone stale.
fn check_domains(coverage: &[DomainCoverage], diag: &mut DiagnosticBag) {
    for domain in coverage {
        if domain.branches.len() > 1 {
            let branches: Vec<&str> = domain.branches.iter().map(String::as_str).collect();
            diag.push(Diagnostic::warning(
                Path::new(&domain.domain),
                format!(
                    "域 \"{}\" 的 topic 分散在多个分支下：{}",
                    domain.domain,
                    branches.join("、")
                ),
            ));
        }
    }
}
