//! 机器面：`IaReport` 的 JSON 形。
//!
//! 人读输出（`render.rs`）是给终端看的，机器要消费只能去解析文本。本模块给出
//! 另一条出口，让 `ia` 能接进 CI、被别的工具读、做历史对比（"这次覆盖度比上次
//! 少了什么"）。
//!
//! # 这是一份契约
//!
//! 字段名与嵌套一旦发布，下游就会依赖。所以结构在这里**逐字段显式写出**，不用
//! `#[derive(Serialize)]` 挂在各模块的结构体上：那样 Rust 侧一次改名就会静默改掉
//! 外部契约，而改名是重构里最不起眼的动作。要加字段可以（下游按名取值），
//! 改名和删字段要当作破坏性变更，同时抬 `schema_version`。
//!
//! # 两条与人读输出不同的取法，都是刻意的
//!
//! - **不受 `--details` / `--depth` 影响，永远全量。** 那两个开关解决的是终端里
//!   一屏放不下，机器没有这个问题；让输出随开关变形，等于给下游一个会随命令行
//!   改变的契约。
//! - **路径一律相对 kb 根**（`topics/ai/x.dita`、`maps/domains/ai.ditamap`），
//!   而不是人读输出里孤儿那一行的"相对 topics 根"。JSON 里 map 与 topic 的路径
//!   会并排出现，同一个基准才拼得起来；绝对路径则会把机器的构建目录写进产物，
//!   历史对比就无从比起。
//!
//! 对象的键按字典序输出（`serde_json` 默认 `BTreeMap`）。顺序因此是确定的，
//! 两次运行的产物可以直接 diff；下游按名取值，不该依赖顺序。

use crate::{DuplicateKind, IaReport, Node, State};
use serde_json::{Map, Value, json};
use std::path::Path;

/// 契约版本。加字段不动它；改名、删字段、改语义要抬。
const SCHEMA_VERSION: u32 = 1;

/// 整份报告的 JSON 形。
#[must_use]
pub fn json_report(report: &IaReport) -> Value {
    json!({
        "schema_version": SCHEMA_VERSION,
        "totals": {
            "topics": report.topics.len(),
            "topics_on_skeleton": report.skeleton.iter().map(Node::total_topics).sum::<usize>(),
            "planned_nodes": report.skeleton.iter().map(count_nodes).sum::<usize>(),
            "maps_consulted": report.consulted.len(),
        },
        "skeleton": report.skeleton.iter().map(node).collect::<Vec<_>>(),
        "branches": report.branch_stats.iter().map(|b| json!({
            "name": b.name,
            "topics": b.topics,
            "has_landscape": b.has_landscape,
            "by_type": counts(&b.by_type),
            "by_maturity": counts(&b.by_maturity),
            "by_volatility": counts(&b.by_volatility),
        })).collect::<Vec<_>>(),
        "plans": report.plans.iter().map(|p| json!({
            "key": p.key,
            "planned": p.planned,
            "planned_total": p.planned_total,
            "built": p.built,
            "matched_branch": p.matched_branch,
        })).collect::<Vec<_>>(),
        "coverage": report.coverage.iter().map(|c| json!({
            "domain": c.domain,
            "planned": set(&c.planned),
            "covered": set(&c.covered),
            "blind": set(&c.blind),
            "outside_plan": set(&c.outside_plan),
            "branches": set(&c.branches),
            "topics": c.topics,
            "percent": c.percent(),
        })).collect::<Vec<_>>(),
        "benchmarks": report.benchmarks.iter().map(|b| json!({
            "key": b.key,
            "anchor": b.anchor,
            "last_benchmarked": b.last_benchmarked,
            "cadence": b.cadence,
            // 事件触发（无日历到期）是 null，不是 0——0 会被读成"已到期"
            "due_months": b.due_months(),
        })).collect::<Vec<_>>(),
        "value_usage": report.value_usage.iter().map(|u| json!({
            "attribute": u.attribute,
            "used": counts(&u.used),
            "unused": u.unused.iter().collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "exceptions": exceptions(report),
        "diagnostics": report.diagnostics.items.iter().map(|d| json!({
            "severity": if d.is_error() { "error" } else { "warning" },
            "path": rel(d.path(), report),
            "message": d.message(),
        })).collect::<Vec<_>>(),
    })
}

/// 需要人去处理的那些。与人读输出的「需要处理」段同一批事实，
/// 只是这里逐项给结构，而不是拼成一句话。
fn exceptions(report: &IaReport) -> Value {
    let in_branches: usize = report.branch_stats.iter().map(|b| b.topics).sum();
    json!({
        "orphans": report.orphans.iter().map(|p| rel(p, report)).collect::<Vec<_>>(),
        "topics_outside_branches": report.topics.len().saturating_sub(in_branches),
        "duplicate_topicrefs": report.duplicate_refs.iter().map(|d| json!({
            "kind": match d.kind {
                DuplicateKind::SameMap => "same_map",
                DuplicateKind::SameTree => "same_tree",
            },
            "scope": rel(&d.scope, report),
            "topic": rel(&d.topic, report),
            "via": d.via.iter().map(|p| rel(p, report)).collect::<Vec<_>>(),
            "count": d.count,
        })).collect::<Vec<_>>(),
        "blind_dimensions": report.coverage.iter().map(|c| c.blind.len()).sum::<usize>(),
        "outside_plan": report.coverage.iter()
            .filter(|c| !c.outside_plan.is_empty())
            .map(|c| json!({ "domain": c.domain, "dimensions": set(&c.outside_plan) }))
            .collect::<Vec<_>>(),
        "empty_leaves_by_branch": report.empty_leaves_by_branch.iter()
            .map(|(branch, count)| json!({ "branch": branch, "count": count }))
            .collect::<Vec<_>>(),
        "diagnostics": {
            "errors": report.diagnostics.error_count(),
            "warnings": report.diagnostics.warning_count(),
        },
        // 词表读不到时上面几项是"没检查"而不是"没问题"，下游必须能分辨
        "vocab_loaded": report.vocab_loaded,
    })
}

fn node(node: &Node) -> Value {
    json!({
        "key": node.key,
        "label": node.label,
        "state": match node.state {
            State::Unbuilt => "unbuilt",
            State::InProgress => "in_progress",
            State::Done => "done",
            State::NotApplicable => "not_applicable",
        },
        "topics": node.topics,
        "unplaced": node.unplaced,
        "outside": node.outside,
        // 没有概览的节点是 null，不是 0/0——后者会被读成"规划了零维度"
        "coverage": node.coverage.map(|(covered, planned)| json!({
            "covered": covered,
            "planned": planned,
        })),
        "benchmark": node.benchmark,
        "children": node.children.iter().map(self::node).collect::<Vec<_>>(),
    })
}

fn count_nodes(node: &Node) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

fn counts(map: &std::collections::BTreeMap<String, usize>) -> Map<String, Value> {
    map.iter()
        .map(|(k, v)| (k.clone(), Value::from(*v)))
        .collect()
}

fn set(values: &std::collections::BTreeSet<String>) -> Vec<&str> {
    values.iter().map(String::as_str).collect()
}

/// 相对 kb 根（`topics_root` 的上一级）。裁不掉时原样输出——宁可露出绝对路径，
/// 也不静默截断成一个指不到文件的相对路径。
fn rel(path: &Path, report: &IaReport) -> String {
    let base = report
        .topics_root
        .parent()
        .unwrap_or(report.topics_root.as_path());
    path.strip_prefix(base)
        .unwrap_or(path)
        .display()
        .to_string()
}
