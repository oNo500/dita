mod consistency;
mod domain;
mod governance;
mod orphan;
mod paint;
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
    let coverage = stats::domain_coverage(&branch_map, &topics);

    let mut plans = Vec::new();
    let mut benchmarks = Vec::new();
    let mut value_usage = Vec::new();
    let vocab_loaded = match vocab {
        Some(path) if path.exists() => {
            let (vocabulary, diag) = dita_vocab::parse_vocab(path)?;
            diagnostics.items.extend(diag.items);
            check_values(&vocabulary, &topics, &mut diagnostics);
            plans = governance::branch_plans(&vocabulary, &branch_map, &branch_stats);
            benchmarks = governance::benchmarks(&vocabulary);
            value_usage = governance::value_usage(&vocabulary, &topics);
            true
        }
        _ => false,
    };

    let mut skeleton = Vec::new();
    if let Some(path) = vocab.filter(|p| p.exists()) {
        let (vocabulary, _) = dita_vocab::parse_vocab(path)?;
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
    })
}

/// Check tagged values against the subject scheme — the vocabulary is the only
/// source of legal values, so this is the one place that knows them.
fn check_values(vocab: &dita_vocab::Vocabulary, topics: &[TopicMeta], diag: &mut DiagnosticBag) {
    // domain must name a subject key — it is the only link from a topic to the
    // taxonomy, and a typo silently detaches the topic from the skeleton
    let subject_keys = vocab
        .subject("subject")
        .map(dita_vocab::Subject::all_keys)
        .unwrap_or_default();
    for meta in topics {
        if let Some(domain) = &meta.domain {
            if !subject_keys.is_empty() && !subject_keys.contains(domain) {
                diag.push(Diagnostic::error(
                    &meta.path,
                    format!("domain 值 \"{domain}\" 不是词表主题键"),
                ));
            }
        }
        if let Some(legal) = vocab.legal_values("dimension") {
            for value in &meta.dimensions {
                if !legal.contains(value) {
                    diag.push(Diagnostic::error(
                        &meta.path,
                        format!("@dimension 值 \"{value}\" 不在词表中"),
                    ));
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
            *illegal.entry(d.path().to_path_buf()).or_default() += 1;
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
