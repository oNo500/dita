//! 骨架树的渲染（与 `render.rs` 同属渲染层，`print_stdout` 的豁免理由见那里）。
#![allow(clippy::print_stdout)]

use crate::{BenchmarkEntry, BranchPlan, DomainCoverage, Paint, TopicRef};
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
    pub paint: Paint,
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
    let plain_leaves = nodes
        .iter()
        .all(|n| matches!(n, MapNode::TopicRef(t) if t.children.is_empty()));
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
                // keyref 未解析或纯分组的 topicref：没有文件可标注，只画名字。
                // 本库的 map 不用这两种写法，这条分支是给上游源留的
                let Some(href) = &t.href else {
                    let label = t
                        .nav_title
                        .clone()
                        .or_else(|| t.keyref.clone())
                        .unwrap_or_else(|| "(unnamed)".to_string());
                    println!("{prefix}{conn}{label}");
                    print_nodes(&t.children, &child_prefix, ann);
                    continue;
                };
                let path = href.canonicalize().unwrap_or_else(|_| href.clone());
                let name = href.file_name().and_then(|s| s.to_str()).unwrap_or("?");
                if !href.exists() {
                    println!("{prefix}{conn}✗ {name}   ← 文件不存在");
                    continue;
                }
                // topic 节点默认显示标题（人读的），文件名是 ASCII kebab 契约，
                // 不是给人看的——见 dita-tools ia 的 title-display 设计
                let label = ann.topics.get(&path).map_or_else(
                    || name.to_string(),
                    |meta| TopicRef::from_meta(meta).label(ann.paint, ann.full),
                );
                println!("{prefix}{conn}{label}{}", topic_note(&path, ann));
                print_nodes(&t.children, &child_prefix, ann);
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
        if meta.maturity.as_deref() != Some("curated")
            && meta.maturity.as_deref() != Some("verified")
        {
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
            MapNode::TopicRef(t) => usize::from(t.href.is_some()) + count_topics(&t.children),
            MapNode::TopicHead(h) => count_topics(&h.children),
            MapNode::MapRef(m) => count_topics(&m.children),
        })
        .sum()
}
