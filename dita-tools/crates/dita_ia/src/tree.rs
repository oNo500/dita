use crate::{BenchmarkEntry, BranchPlan, DomainCoverage};
use dita_ast::{DitaMap, MapNode, ProcessingRole, TopicMeta};
use std::{collections::BTreeMap, path::PathBuf};

/// Everything the skeleton needs to annotate itself.
///
/// The skeleton is the view — not a section of it. Numbers that describe a
/// branch belong on that branch's line, where they are read together with it;
/// pulled into separate tables they become a report, and the shape of the
/// library stops being visible.
pub struct Annotations<'a> {
    /// Print every leaf instead of collapsing long uniform runs.
    pub full: bool,
    pub topics: BTreeMap<PathBuf, &'a TopicMeta>,
    /// branch label → what the vocabulary plans for it
    pub plans: BTreeMap<String, &'a BranchPlan>,
    /// branch label → its benchmark registry entry
    pub benchmarks: BTreeMap<String, &'a BenchmarkEntry>,
    /// landscape topic path → the coverage of the domain it declares
    pub coverage: BTreeMap<PathBuf, &'a DomainCoverage>,
    /// topics carrying values the vocabulary does not define
    pub illegal: BTreeMap<PathBuf, usize>,
}

pub fn print_skeleton(map: &DitaMap, ann: &Annotations) {
    let file = map.path.file_name().and_then(|s| s.to_str()).unwrap_or("?");
    println!("{}  ← {file}", map.title);
    print_nodes(&map.children, "", ann);
}

/// Beyond this many plain topics in a row, the names stop being skeleton and
/// start being a directory listing. The count already carries the structural
/// fact; the names are one `--details` away.
const COLLAPSE_AFTER: usize = 6;

fn print_nodes(nodes: &[MapNode], prefix: &str, ann: &Annotations) {
    let count = nodes.len();
    let plain_leaves = nodes.iter().all(|n| matches!(n, MapNode::TopicRef(_)));
    let collapse = !ann.full && plain_leaves && count > COLLAPSE_AFTER;
    for (i, node) in nodes.iter().enumerate() {
        if collapse && i == 3 {
            println!("{prefix}└── …（共 {count} 篇，--details 列全）");
            return;
        }
        let is_last = i == count - 1;
        let conn = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };

        match node {
            MapNode::TopicRef(t) => {
                let path = t.href.canonicalize().unwrap_or_else(|_| t.href.clone());
                let name = t.href.file_name().and_then(|s| s.to_str()).unwrap_or("?");
                if !t.href.exists() {
                    println!("{prefix}{conn}✗ {name}   ← 文件不存在");
                    continue;
                }
                println!("{prefix}{conn}{name}{}", topic_note(&path, ann));
            }
            MapNode::TopicHead(h) => {
                // a wrapper around one same-named mapref exists only to give the
                // map a navigation node; drawing both would show one branch twice
                if let [MapNode::MapRef(m)] = h.children.as_slice() {
                    if m.title.as_deref() == Some(h.nav_title.as_str()) {
                        println!(
                            "{prefix}{conn}{}{}",
                            h.nav_title,
                            branch_note(&h.nav_title, &m.children, ann)
                        );
                        print_nodes(&m.children, &child_prefix, ann);
                        continue;
                    }
                }
                println!(
                    "{prefix}{conn}{}{}",
                    h.nav_title,
                    branch_note(&h.nav_title, &h.children, ann)
                );
                print_nodes(&h.children, &child_prefix, ann);
            }
            MapNode::MapRef(m) => {
                if m.processing_role == ProcessingRole::ResourceOnly {
                    let name = m.href.file_name().and_then(|s| s.to_str()).unwrap_or("?");
                    println!("{prefix}{conn}{name}   ← 词表，不进导航");
                    continue;
                }
                let label = m.title.clone().unwrap_or_else(|| {
                    m.href
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("?")
                        .to_string()
                });
                println!(
                    "{prefix}{conn}{label}{}",
                    branch_note(&label, &m.children, ann)
                );
                print_nodes(&m.children, &child_prefix, ann);
            }
        }
    }
}

/// What a branch line says about itself: how much it holds, how much the
/// vocabulary plans for it, and whether its taxonomy is due for re-benchmarking.
fn branch_note(label: &str, children: &[MapNode], ann: &Annotations) -> String {
    let built = count_topics(children);
    let mut parts = vec![if built == 0 {
        "空".to_string()
    } else {
        format!("{built} 篇")
    }];

    if let Some(plan) = ann.plans.get(label) {
        parts.push(if plan.planned_total == plan.planned.len() {
            format!("规划 {}", plan.planned.len())
        } else {
            format!("规划 {}↓{}", plan.planned.len(), plan.planned_total)
        });
    }
    if let Some(bm) = ann.benchmarks.get(label) {
        if let (Some(date), Some(months)) = (&bm.last_benchmarked, bm.due_months()) {
            parts.push(format!("对标 {date}+{months}mo"));
        }
    }
    format!("   {}", parts.join(" · "))
}

/// What a topic line says: only what deviates or needs acting on. Marking every
/// topic `curated stable` would drown the skeleton in the unremarkable.
fn topic_note(path: &PathBuf, ann: &Annotations) -> String {
    let mut parts = Vec::new();
    if let Some(meta) = ann.topics.get(path) {
        if meta.maturity.as_deref() != Some("curated") && meta.maturity.as_deref() != Some("verified") {
            parts.push(meta.maturity.clone().unwrap_or_else(|| "未标成熟度".into()));
        }
        if meta.volatility.is_none() {
            parts.push("未标时效".to_string());
        }
    }
    if let Some(cov) = ann.coverage.get(path) {
        parts.push(format!(
            "全景 {} {}/{}",
            cov.domain,
            cov.covered.len(),
            cov.planned.len()
        ));
    }
    if let Some(n) = ann.illegal.get(path) {
        parts.push(format!("⚠ {n} 个非法值"));
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!("   {}", parts.join(" · "))
    }
}

#[must_use]
pub fn count_topics(nodes: &[MapNode]) -> usize {
    nodes
        .iter()
        .map(|n| match n {
            MapNode::TopicRef(_) => 1,
            MapNode::TopicHead(h) => count_topics(&h.children),
            MapNode::MapRef(m) => count_topics(&m.children),
        })
        .sum()
}
