use dita_ia::{IaReport, build_report};
use std::path::{Path, PathBuf};

const MINI: &str = "tests/fixtures/mini";

fn report() -> IaReport {
    let root = Path::new(MINI);
    build_report(
        &[root.join("maps/root.ditamap")],
        &root.join("topics"),
        Some(&root.join("maps")),
        Some(&root.join("vocab/scheme.ditamap")),
    )
    .expect("report failed")
}

fn report_without_vocab() -> IaReport {
    let root = Path::new(MINI);
    build_report(
        &[root.join("maps/root.ditamap")],
        &root.join("topics"),
        Some(&root.join("maps")),
        Some(&PathBuf::from("does/not/exist.ditamap")),
    )
    .expect("report failed")
}

#[test]
fn coverage_counts_only_planned_dimensions() {
    // same semantics as dimension-coverage.py: dim-usage is covered and planned,
    // dim-nonexistent is covered but not planned, so it lifts neither numerator
    // nor denominator
    let report = report();
    let demo = report
        .coverage
        .iter()
        .find(|c| c.domain == "demo")
        .expect("demo domain");
    assert_eq!(demo.planned.len(), 3);
    // dim-concept 与 dim-usage 直接归 demo；dim-security 来自孙键 demo-b1，按分类法上卷
    assert_eq!(demo.covered.len(), 3);
    assert!(demo.blind.is_empty());
    // 规划外的覆盖不进分子、也不进分母
    assert!(demo.outside_plan.contains("dim-nonexistent"));
    assert_eq!(demo.percent(), 100);
}

#[test]
fn coverage_records_which_branch_the_domain_sits_in() {
    // this is what the map tree adds over the script: the script knows only
    // what each topic declares about itself
    let report = report();
    let demo = report.coverage.iter().find(|c| c.domain == "demo").unwrap();
    assert_eq!(
        demo.branches.iter().map(String::as_str).collect::<Vec<_>>(),
        ["演示分支"]
    );
}

#[test]
fn illegal_values_are_errors_naming_the_attribute() {
    let report = report();
    let messages: Vec<&str> = report
        .diagnostics
        .items
        .iter()
        .filter(|d| d.is_error())
        .map(dita_diagnostics::Diagnostic::message)
        .collect();
    assert!(
        messages.iter().any(|m| m.contains("dim-nonexistent")),
        "illegal @dimension must be reported: {messages:?}"
    );
    assert!(
        messages.iter().any(|m| m.contains("maturity")),
        "illegal @maturity must be reported: {messages:?}"
    );
}

#[test]
fn without_a_vocabulary_values_are_not_guessed_at() {
    // skipping the check and saying so beats inventing a legal-value list
    let report = report_without_vocab();
    assert!(!report.vocab_loaded);
    assert!(
        !report
            .diagnostics
            .items
            .iter()
            .any(|d| d.message().contains("不在词表中")),
        "no vocabulary means no value verdicts"
    );
}

#[test]
fn empty_branch_is_listed_with_zero_topics() {
    // the whole point of the IA view: a branch that exists and holds nothing
    // has to stay visible
    let report = report();
    let empty = report
        .branch_stats
        .iter()
        .find(|b| b.name == "空分支")
        .expect("empty branch must appear");
    assert_eq!(empty.topics, 0);
}

#[test]
fn branch_stats_break_topics_down_by_type_and_maturity() {
    let report = report();
    let demo = report
        .branch_stats
        .iter()
        .find(|b| b.name == "演示分支")
        .expect("demo branch");
    assert_eq!(demo.topics, 5);
    assert_eq!(demo.by_type.get("concept"), Some(&5));
    assert_eq!(demo.by_maturity.get("curated"), Some(&4));
}

#[test]
fn plan_matches_branches_by_map_file_name() {
    // the scheme keys branches in English and the maps title them in Chinese;
    // domains/<key>.ditamap is the only thing carrying the correspondence
    let report = report();
    let demo = report
        .plans
        .iter()
        .find(|p| p.key == "demo")
        .expect("demo plan");
    assert_eq!(demo.matched_branch.as_deref(), Some("演示分支"));
    assert_eq!(demo.built, 5);
    assert_eq!(demo.planned.len(), 2, "direct sub-topics");
    assert_eq!(demo.planned_total, 3, "including the nested one");
}

#[test]
fn unmatched_subject_key_is_not_counted_as_zero() {
    // silently reporting "0 built" for a key with no map would read as "nothing
    // written yet" when the truth is "nothing to compare against"
    let report = report();
    let nomap = report
        .plans
        .iter()
        .find(|p| p.key == "nomap")
        .expect("nomap plan");
    assert_eq!(nomap.matched_branch, None);
}

#[test]
fn benchmark_entries_carry_dates_and_cadence() {
    let report = report();
    let demo = report
        .benchmarks
        .iter()
        .find(|b| b.key == "bm-demo")
        .expect("bm-demo");
    assert_eq!(demo.last_benchmarked.as_deref(), Some("2026-01-01"));
    assert_eq!(demo.due_months(), Some(6));
    // anchors are element text, dates are @value — both forms must be read
    assert_eq!(demo.anchor.as_deref(), Some("某个外部对标锚点"));

    let on_trigger = report
        .benchmarks
        .iter()
        .find(|b| b.key == "bm-empty")
        .unwrap();
    assert_eq!(
        on_trigger.due_months(),
        None,
        "event-triggered has no calendar expiry"
    );
}

#[test]
fn unused_controlled_values_are_listed() {
    let report = report();
    let maturity = report
        .value_usage
        .iter()
        .find(|u| u.attribute == "maturity")
        .expect("maturity usage");
    assert_eq!(maturity.used.get("curated"), Some(&4));
    assert!(
        maturity.unused.contains("draft"),
        "a value the vocabulary defines but no topic uses must show up"
    );

    let dimension = report
        .value_usage
        .iter()
        .find(|u| u.attribute == "dimension")
        .unwrap();
    // an illegal value is not usage: it is already an error elsewhere
    assert!(!dimension.used.contains_key("dim-nonexistent"));
}

// ── 骨架（设计见 docs/plans/2026-08-15-skeleton-design.md）────────────────

fn node<'a>(nodes: &'a [dita_ia::Node], key: &str) -> &'a dita_ia::Node {
    nodes
        .iter()
        .find(|n| n.key == key)
        .unwrap_or_else(|| panic!("node {key}"))
}

#[test]
fn skeleton_is_the_subject_tree_with_content_hung_on_it() {
    let report = report();
    let demo = node(&report.skeleton, "demo");
    assert_eq!(
        demo.children.len(),
        2,
        "planned sub-topics are listed even when empty"
    );
    assert_eq!(demo.state, dita_ia::State::InProgress);
    // planned but unwritten sub-topics must be visible — that is the point
    assert_eq!(
        node(&demo.children, "demo-a").state,
        dita_ia::State::Unbuilt
    );
}

#[test]
fn topics_without_a_domain_land_in_the_unplaced_bucket() {
    // the only link from a topic to a subject key is <data name="domain">;
    // without it nothing can place the topic, and hiding that would hide the
    // largest structural gap in the library
    let report = report();
    let demo = node(&report.skeleton, "demo");
    assert_eq!(
        demo.topics.len() + demo.unplaced.len(),
        4,
        "nested 归在孙键，不计入 demo 节点自身"
    );
}

#[test]
fn branch_without_a_vocabulary_key_is_not_applicable_not_unbuilt() {
    // an organisational branch (the glossary) is not a subject: it can never be
    // "done", and calling it unbuilt would be a permanent false alarm
    let mut report = report();
    report
        .skeleton
        .retain(|n| n.state == dita_ia::State::NotApplicable);
    assert!(
        report.skeleton.iter().all(|n| n.children.is_empty()),
        "unkeyed branches carry topics, not planned children"
    );
}

#[test]
fn an_empty_planned_branch_stays_unbuilt() {
    let report = report();
    assert_eq!(
        node(&report.skeleton, "empty").state,
        dita_ia::State::Unbuilt
    );
}

#[test]
fn a_typo_in_domain_falls_into_the_bucket_and_errors() {
    // the failure mode this guards: "declared anything" counted as placed, so a
    // typo'd domain hung nowhere and the topic vanished from the skeleton —
    // worse than undeclared, which at least stays visible in the bucket
    let report = report();
    let demo = node(&report.skeleton, "demo");
    assert!(
        demo.unplaced.iter().any(|n| n == "bogus-domain.dita"),
        "typo'd domain must stay visible in the unplaced bucket: {:?}",
        demo.unplaced
    );
    assert!(
        report
            .diagnostics
            .items
            .iter()
            .any(|d| d.is_error() && d.message().contains("not-a-subject-key")),
        "and be reported as an error naming the bogus value"
    );
}

#[test]
fn coverage_rolls_up_the_subject_tree() {
    // a subject scheme is a taxonomy: filing under demo-b1 (a grandchild) covers
    // the dimension for demo. Exact matching would force a near-identical
    // landscape under every leaf.
    let report = report();
    let demo = report.coverage.iter().find(|c| c.domain == "demo").unwrap();
    assert!(
        demo.covered.contains("dim-security"),
        "a grandchild's dimension must count toward the ancestor's plan"
    );
    // demo 自身 3 篇（landscape/good/illegal）＋ 孙键 demo-b1 的 1 篇；
    // bogus-domain 的 domain 不是词表键，不计入任何域
    assert_eq!(demo.topics, 4, "rolled-up topics are counted too");
}

// ── R17: domain 必须是 subjectScheme 已注册的 subject key ──────────────────

#[test]
fn r17_a_registered_domain_value_is_not_flagged() {
    // good.dita / landscape.dita declare domain="demo", and nested.dita
    // declares the grandchild key "demo-b1" — both are registered subject
    // keys, so none of these three files should ever carry an R17 error.
    // (bogus-domain.dita is the deliberate counter-example, covered below.)
    let report = report();
    let r17_paths: Vec<String> = report
        .diagnostics
        .items
        .iter()
        .filter(|d| d.is_error() && d.message().contains("R17"))
        .map(|d| d.path().display().to_string())
        .collect();
    for clean in ["good.dita", "landscape.dita", "nested.dita"] {
        assert!(
            !r17_paths.iter().any(|p| p.ends_with(clean)),
            "{clean} declares a registered domain and must not raise R17: {r17_paths:?}"
        );
    }
}

#[test]
fn r17_an_unregistered_domain_value_errors_with_a_fix_hint() {
    // bogus-domain.dita declares domain="not-a-subject-key", which names no
    // subjectdef anywhere in the scheme. The message must name the offending
    // value and point at the fix (register it, or use a registered value) —
    // a bare "illegal" tells the author nothing about what to do next.
    let report = report();
    let message = report
        .diagnostics
        .items
        .iter()
        .find(|d| d.is_error() && d.message().contains("not-a-subject-key"))
        .map(dita_diagnostics::Diagnostic::message)
        .expect("bogus domain must be reported");
    assert!(message.contains("R17"), "message must cite R17: {message}");
    assert!(
        message.contains("注册"),
        "message must hint at registering the key or using a registered one: {message}"
    );
}

#[test]
fn r17_empty_leaves_by_branch_counts_unclaimed_leaves_per_branch() {
    // the reverse report: demo-a, empty-a and nomap are registered leaves no
    // topic ever names as domain — the tree's empty leaves, counted per
    // top-level branch. demo-b1 is a leaf too but nested.dita claims it, so
    // "demo" must count only demo-a (1), not demo-b1 as well (would be 2).
    // "nomap" has no children of its own, so it is simultaneously a
    // top-level branch and the one leaf under it.
    let report = report();
    let by_branch = &report.empty_leaves_by_branch;
    let count = |branch: &str| by_branch.iter().find(|(b, _)| b == branch).map(|(_, n)| *n);
    assert_eq!(
        count("demo"),
        Some(1),
        "demo-b1 is claimed, so only demo-a should count: {by_branch:?}"
    );
    assert_eq!(
        count("empty"),
        Some(1),
        "empty-a is unclaimed: {by_branch:?}"
    );
    assert_eq!(
        count("nomap"),
        Some(1),
        "nomap is a leaf and a branch at once: {by_branch:?}"
    );
    let total: usize = by_branch.iter().map(|(_, n)| n).sum();
    assert_eq!(total, 3);
}

#[test]
fn r17_empty_leaves_by_branch_is_empty_without_a_vocabulary() {
    // no vocabulary means no "ought" to compare against — silence, not a
    // false claim that everything is covered.
    let report = report_without_vocab();
    assert!(report.empty_leaves_by_branch.is_empty());
}
