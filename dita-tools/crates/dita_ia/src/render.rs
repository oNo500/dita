//! 文本渲染层：把 `IaReport` 画成终端里的骨架。
//!
//! 这里是本 crate 唯一允许写 stdout 的地方——`print_stdout` 那条 lint 是对的
//! （库不该替调用方决定输出到哪），所以渲染被隔离在这一个模块里，计算层不受豁免。
//!
//! 终局：等 `--format json` 落地（见 `docs/plans/2026-08-15-topic-parser-and-ia-depth.md`
//! Task 4），渲染应抽成 formatter 并整体搬进 `dita_cli`——库产出数据、应用决定呈现。
//! 在那之前，本模块的 allow 是有边界、有落款的技术债，不是默认放行。
#![allow(clippy::print_stdout)]

use crate::{IaReport, Node, Paint, State, tree};
use std::path::PathBuf;

/// The skeleton is the view: the subject tree, with content hung on it and each
/// node's state on its own line. Design: docs/plans/2026-08-15-skeleton-design.md
pub fn print_report(report: &IaReport, details: bool, depth: Option<usize>) {
    let paint = Paint::detect();
    println!();
    if report.skeleton.is_empty() {
        // no vocabulary means no "ought", so fall back to what the maps say
        let ann = annotations(report, details);
        for map in &report.display {
            tree::print_skeleton(map, &ann);
        }
        println!("\n（未读到词表：只能显示 maps 的实际结构，无法对照规划）");
    } else {
        let placed: usize = report.skeleton.iter().map(Node::total_topics).sum();
        let planned: usize = report.skeleton.iter().map(count_nodes).sum();
        let total = report.topics.len();
        println!("知识体系   全库 {total} 篇（骨架内 {placed}）· 词表规划 {planned} 个主题节点\n");
        let last = report.skeleton.len().saturating_sub(1);
        for (i, node) in report.skeleton.iter().enumerate() {
            print_node(node, "", i == last, paint, depth, 0);
        }
        println!(
            "\n{}",
            paint.dim(
                "○ 未建   ◐ 进行中   ● 完成（有全景且零盲区）   · 不适用   ⚠ 有问题   ⏰ 待复核"
            )
        );
    }

    print_exceptions(report);

    if details {
        print_branches(report);
        print_plans(report);
        print_coverage(report);
        print_benchmarks(report);
        print_value_usage(report);
    } else if has_details(report) {
        println!(
            "\n{}",
            paint.dim("（--details 展开分支统计、对标登记、受控值使用；--depth N 限制层数）")
        );
    }
}

fn count_nodes(node: &Node) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

fn print_node(
    node: &Node,
    prefix: &str,
    is_last: bool,
    paint: Paint,
    depth: Option<usize>,
    level: usize,
) {
    let conn = if is_last { "└── " } else { "├── " };
    let child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });

    let symbol = match node.state {
        State::Unbuilt => paint.dim(node.state.symbol()),
        State::InProgress => paint.yellow(node.state.symbol()),
        State::Done => paint.green(node.state.symbol()),
        State::NotApplicable => node.state.symbol().to_string(),
    };
    let name = node.label.as_deref().unwrap_or(&node.key);
    let mut notes = Vec::new();
    if !node.children.is_empty() {
        notes.push(format!("{}/{}", node.built_children(), node.children.len()));
    }
    let own = node.topics.len() + node.unplaced.len();
    if own > 0 {
        notes.push(format!("{own} 篇"));
    }
    if let Some((covered, planned)) = node.coverage {
        notes.push(format!("全景 {covered}/{planned}"));
    }
    if let Some(bm) = &node.benchmark {
        notes.push(paint.dim(bm));
    }
    let line = if notes.is_empty() {
        format!("{prefix}{conn}{symbol} {name}")
    } else {
        format!("{prefix}{conn}{symbol} {name}   {}", notes.join(" · "))
    };
    println!(
        "{line}{}",
        if node.state == State::Unbuilt && node.children.is_empty() {
            paint.dim("   —")
        } else {
            String::new()
        }
    );

    if depth.is_some_and(|d| level + 1 >= d) {
        return;
    }

    let extra = node.topics.len() + usize::from(!node.unplaced.is_empty());
    let total = node.children.len() + extra;
    let mut printed = 0;

    for child in &node.children {
        printed += 1;
        print_node(
            child,
            &child_prefix,
            printed == total,
            paint,
            depth,
            level + 1,
        );
    }
    for name in &node.topics {
        printed += 1;
        let conn = if printed == total {
            "└──"
        } else {
            "├──"
        };
        println!("{child_prefix}{conn} {name}");
    }
    if !node.unplaced.is_empty() {
        printed += 1;
        let conn = if printed == total {
            "└──"
        } else {
            "├──"
        };
        println!(
            "{child_prefix}{conn} {} {} 篇未归子主题：{}",
            paint.red("⚠"),
            node.unplaced.len(),
            node.unplaced.join("、")
        );
    }
}

fn annotations(report: &IaReport, details: bool) -> tree::Annotations<'_> {
    let mut illegal: std::collections::BTreeMap<PathBuf, usize> = std::collections::BTreeMap::new();
    for d in &report.diagnostics.items {
        if d.is_error() && d.message().contains("不在词表中") {
            *illegal.entry(d.path().to_owned()).or_default() += 1;
        }
    }
    tree::Annotations {
        full: details,
        topics: report.topics.iter().map(|t| (t.path.clone(), t)).collect(),
        plans: report
            .plans
            .iter()
            .filter_map(|p| p.matched_branch.clone().map(|b| (b, p)))
            .collect(),
        benchmarks: report
            .benchmarks
            .iter()
            .filter_map(|b| {
                let key = b.key.strip_prefix("bm-")?;
                let plan = report.plans.iter().find(|p| p.key == key)?;
                Some((plan.matched_branch.clone()?, b))
            })
            .collect(),
        coverage: report
            .coverage
            .iter()
            .filter_map(|c| {
                let landscape = report.topics.iter().find(|t| {
                    t.domain.as_deref() == Some(&c.domain) && !t.planned_dimensions.is_empty()
                })?;
                Some((landscape.path.clone(), c))
            })
            .collect(),
        illegal,
    }
}

fn has_details(report: &IaReport) -> bool {
    !report.plans.is_empty() || !report.benchmarks.is_empty() || !report.coverage.is_empty()
}

/// Only what needs acting on, one line each. Silence means nothing is wrong.
fn print_exceptions(report: &IaReport) {
    let mut lines = Vec::new();
    if !report.orphans.is_empty() {
        lines.push(format!(
            "孤儿 {} 篇（写了但没挂进任何 map）：{}",
            report.orphans.len(),
            report
                .orphans
                .iter()
                .map(|p| p
                    .strip_prefix(&report.topics_root)
                    .unwrap_or(p)
                    .display()
                    .to_string())
                .collect::<Vec<_>>()
                .join("、")
        ));
    }
    let unplaced = report
        .topics
        .len()
        .saturating_sub(report.branch_stats.iter().map(|b| b.topics).sum::<usize>());
    if unplaced > 0 {
        lines.push(format!(
            "{unplaced} 篇不在任何分支下（只被交付物 map 引用）"
        ));
    }
    let blind: usize = report.coverage.iter().map(|c| c.blind.len()).sum();
    if blind > 0 {
        lines.push(format!("维度盲区 {blind} 个"));
    }
    for usage in &report.value_usage {
        if !usage.unused.is_empty() {
            lines.push(format!(
                "@{} 有 {} 个受控值从未被用过",
                usage.attribute,
                usage.unused.len()
            ));
        }
    }
    let errs = report.diagnostics.error_count();
    let warns = report.diagnostics.warning_count();
    if errs > 0 || warns > 0 {
        lines.push(format!("诊断 {errs} error / {warns} warning"));
    }
    if !report.vocab_loaded {
        lines.push("未读到词表：规划对照与值检查均已跳过".to_string());
    }

    if lines.is_empty() {
        return;
    }
    println!("\n需要处理：");
    for line in lines {
        println!("  · {line}");
    }
    if errs_present(report) {
        println!("\n诊断明细：");
        for d in &report.diagnostics.items {
            let prefix = if d.is_error() { "❌" } else { "⚠ " };
            println!("  {prefix} {}: {}", d.path().display(), d.message());
        }
    }
}

fn errs_present(report: &IaReport) -> bool {
    report.diagnostics.error_count() > 0 || report.diagnostics.warning_count() > 0
}

fn print_branches(report: &IaReport) {
    if report.branch_stats.is_empty() {
        return;
    }
    println!("\n── 按分支 ──");
    println!(
        "  每个分支手上有什么，用来决定下一批写哪里。「· 无全景」= 该分支尚无声明维度清单的全景 topic。"
    );
    let width = report
        .branch_stats
        .iter()
        .map(|b| display_width(&b.name))
        .max()
        .unwrap_or(0);
    for b in &report.branch_stats {
        let name = pad(&b.name, width);
        if b.topics == 0 {
            println!("  {name} 空");
            continue;
        }
        println!(
            "  {name} {:>2} 篇   类型 {}   成熟度 {}   时效 {}{}",
            b.topics,
            render(&b.by_type),
            render(&b.by_maturity),
            render(&b.by_volatility),
            if b.has_landscape {
                ""
            } else {
                "   · 无全景"
            }
        );
    }

    // topics reachable only from a deliverable map hang under no branch, so the
    // rows above do not account for them; saying so keeps the totals honest
    let in_branches: usize = report.branch_stats.iter().map(|b| b.topics).sum();
    let unplaced = report.topics.len().saturating_sub(in_branches);
    if unplaced > 0 {
        println!(
            "  （另有 {unplaced} 篇不属任何分支——只被交付物 map 引用，因此不算孤儿，但也不在上面的统计里）"
        );
    }
}

/// Terminal display width: CJK and full-width punctuation occupy two columns,
/// so `{:<n}` — which counts chars — misaligns any table with mixed scripts.
fn display_width(s: &str) -> usize {
    s.chars()
        .map(|c| {
            let c = c as u32;
            let wide = (0x1100..=0x115F).contains(&c)
                || (0x2E80..=0xA4CF).contains(&c)
                || (0xAC00..=0xD7A3).contains(&c)
                || (0xF900..=0xFAFF).contains(&c)
                || (0xFE30..=0xFE6F).contains(&c)
                || (0xFF00..=0xFF60).contains(&c)
                || (0xFFE0..=0xFFE6).contains(&c);
            usize::from(wide) + 1
        })
        .sum()
}

fn pad(s: &str, width: usize) -> String {
    let mut out = s.to_string();
    out.push_str(&" ".repeat(width.saturating_sub(display_width(s))));
    out
}

fn print_plans(report: &IaReport) {
    if report.plans.is_empty() {
        return;
    }
    println!("\n── 应然对照（词表规划 vs 实际已建）──");
    println!("  词表的主题树是「本该长什么样」。规划了子主题却一篇没有，就是下一批的候选。");
    let width = report
        .plans
        .iter()
        .map(|p| display_width(&p.key))
        .max()
        .unwrap_or(0);
    let mut planned_total = 0;
    let mut unmatched = 0;
    for plan in &report.plans {
        planned_total += plan.planned_total;
        let sub = if plan.planned_total == plan.planned.len() {
            format!("{:>2}", plan.planned.len())
        } else {
            format!("{:>2}（含下级 {}）", plan.planned.len(), plan.planned_total)
        };
        let Some(branch) = plan.matched_branch.as_deref() else {
            unmatched += 1;
            println!(
                "  {}  规划子主题 {sub}   ⚠ 词表键在 maps/ 下找不到同名 map，未能对照",
                pad(&plan.key, width)
            );
            continue;
        };
        let mark = if plan.built == 0 {
            "  ← 规划了但一篇没有"
        } else {
            ""
        };
        println!(
            "  {}  规划子主题 {sub}   实际 {:>2} 篇   （{branch}）{mark}",
            pad(&plan.key, width),
            plan.built,
        );
    }
    println!("  合计：词表规划 {planned_total} 个主题节点（含下级）");
    if unmatched > 0 {
        println!("  其中 {unmatched} 个分支未能与 maps/ 对照——词表键需与领域 map 文件名同名");
    }
}

fn print_benchmarks(report: &IaReport) {
    if report.benchmarks.is_empty() {
        return;
    }
    println!("\n── 分类树防腐（对标登记）──");
    println!("  分类树自己也会过时。这里记的是各分支上次对标的时间与复核档位；");
    println!("  按词表的 policy，到期只 flag 不阻断（expiry-flags-not-blocks）。");
    let width = report
        .benchmarks
        .iter()
        .map(|b| display_width(&b.key))
        .max()
        .unwrap_or(0);
    for b in &report.benchmarks {
        let due = b.due_months().map_or_else(
            || "事件触发（无日历到期）".to_string(),
            |m| format!("{m} 个月后复核"),
        );
        println!(
            "  {}  上次对标 {}   {due}",
            pad(&b.key, width),
            b.last_benchmarked.as_deref().unwrap_or("—"),
        );
    }
}

fn print_value_usage(report: &IaReport) {
    if report.value_usage.is_empty() {
        return;
    }
    println!("\n── 受控值使用情况 ──");
    println!(
        "  定义了却从未被用过的值，要么是规划过早，要么该清理——词表声称的区分，内容里并不存在。"
    );
    for usage in &report.value_usage {
        let used: Vec<String> = usage.used.iter().map(|(v, n)| format!("{v} {n}")).collect();
        println!(
            "  @{:<11} 已用 {:>2} 个：{}",
            usage.attribute,
            usage.used.len(),
            if used.is_empty() {
                "（无）".to_string()
            } else {
                used.join(" / ")
            }
        );
        if !usage.unused.is_empty() {
            let list: Vec<&str> = usage.unused.iter().map(String::as_str).collect();
            let shown = if list.len() > 8 {
                format!("{} …（共 {}）", list[..8].join(" "), list.len())
            } else {
                list.join(" ")
            };
            println!("  {:<12} 未用 {:>2} 个：{shown}", "", usage.unused.len());
        }
    }
}

fn print_coverage(report: &IaReport) {
    println!("\n── 维度覆盖（按技术域，取自各 topic 声明的 domain）──");
    println!("  技术域比分支细（分支 web 下可有 electron / react 各自的全景）。");
    println!("  覆盖度 = 已覆盖 ∩ 规划 / 规划；盲区 = 规划了但还没人写的维度。");
    if report.coverage.is_empty() {
        println!("  没有域声明了 planned-dimension（领域全景未建，或未标 domain）");
    }
    for c in &report.coverage {
        let where_ = if c.branches.is_empty() {
            "（未挂进任何分支）".to_string()
        } else {
            c.branches
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join("、")
        };
        println!(
            "  域 {}：{}/{}（{}%），{} 篇，位于 {where_}",
            c.domain,
            c.covered.len(),
            c.planned.len(),
            c.percent(),
            c.topics,
        );
        if !c.blind.is_empty() {
            println!("     盲区（{}）：{}", c.blind.len(), join(&c.blind));
        }
        if !c.outside_plan.is_empty() {
            println!(
                "     ⚠ 规划外的覆盖（该补进全景或标错了）：{}",
                join(&c.outside_plan)
            );
        }
    }
    if !report.vocab_loaded {
        println!("  （未读到词表，@dimension / @maturity / @volatility 的值合法性未检查）");
    }
}

fn join(values: &std::collections::BTreeSet<String>) -> String {
    values
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ")
}

fn render(counts: &std::collections::BTreeMap<String, usize>) -> String {
    counts
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join(" / ")
}
