use dita_lint::lint_topic;
use dita_vocab::parse_vocab;
use std::path::Path;

fn lint(name: &str) -> dita_diagnostics::DiagnosticBag {
    let (vocab, _) = parse_vocab(Path::new("tests/fixtures/scheme.ditamap")).unwrap();
    lint_topic(Path::new("tests/fixtures").join(name).as_path(), &vocab).unwrap()
}

#[test]
fn clean_topic_passes() {
    let d = lint("clean.dita");
    assert_eq!(
        d.items.len(),
        0,
        "{:?}",
        d.items
            .iter()
            .map(dita_diagnostics::Diagnostic::message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn curated_violations_are_errors() {
    // the promotion gate: curated claims compliance, so failures block
    let d = lint("violations.dita");
    assert!(
        d.error_count() >= 5,
        "缺节×2 + 旧标签 + 手写日期 + 粗体/程度词: {:?}",
        d.items
            .iter()
            .map(dita_diagnostics::Diagnostic::message)
            .collect::<Vec<_>>()
    );
    assert_eq!(d.warning_count(), 0);
    let msgs: Vec<&str> = d
        .items
        .iter()
        .map(dita_diagnostics::Diagnostic::message)
        .collect();
    assert!(msgs.iter().any(|m| m.contains("缺必需节「做法」")));
    assert!(msgs.iter().any(|m| m.contains("已核对")));
    assert!(msgs.iter().any(|m| m.contains("手写日期")));
}

#[test]
fn draft_violations_are_warnings_only() {
    // a draft is free to be unfinished; the report is a worklist, not a gate
    let d = lint("draft-violations.dita");
    assert_eq!(d.error_count(), 0);
    assert!(d.warning_count() >= 1);
}

#[test]
fn genre_must_match_dita_type() {
    let d = lint("wrong-type.dita");
    assert!(
        d.items
            .iter()
            .any(|i| i.message().contains("不能标在 concept 上"))
    );
}
