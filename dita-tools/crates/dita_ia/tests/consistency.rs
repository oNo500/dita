use dita_diagnostics::DiagnosticBag;
use dita_ia::check_group_titles;
use dita_parser::parse_map;
use std::path::Path;

fn check(fixture: &str) -> DiagnosticBag {
    let (map, _) =
        parse_map(Path::new("tests/fixtures").join(fixture).as_path()).expect("parse failed");
    let mut diag = DiagnosticBag::default();
    check_group_titles(&map, &mut diag);
    diag
}

#[test]
fn drifted_wrapper_title_is_reported() {
    // the price of the topichead wrapper is a second copy of the branch name;
    // this check is what keeps that price affordable
    let diag = check("drifted.ditamap");
    assert_eq!(diag.warning_count(), 1);
    assert!(
        diag.items[0].message().contains("旧名字"),
        "the warning must name the stale copy: {}",
        diag.items[0].message()
    );
    assert!(
        !diag.has_errors(),
        "drift is a warning, not a build breaker"
    );
}

#[test]
fn matching_wrapper_title_is_silent() {
    assert_eq!(check("aligned.ditamap").warning_count(), 0);
}
