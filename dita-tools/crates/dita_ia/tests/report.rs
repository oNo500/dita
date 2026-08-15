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
    assert_eq!(demo.covered.len(), 2, "dim-concept and dim-usage");
    assert_eq!(demo.blind.iter().map(String::as_str).collect::<Vec<_>>(), ["dim-security"]);
    assert!(demo.outside_plan.contains("dim-nonexistent"));
    assert_eq!(demo.percent(), 66);
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
    assert_eq!(demo.topics, 3);
    assert_eq!(demo.by_type.get("concept"), Some(&3));
    assert_eq!(demo.by_maturity.get("curated"), Some(&2));
}
