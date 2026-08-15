use dita_vocab::parse_vocab;
use std::path::Path;

fn fixture() -> (dita_vocab::Vocabulary, dita_diagnostics::DiagnosticBag) {
    parse_vocab(Path::new("tests/fixtures/scheme.ditamap")).expect("parse failed")
}

#[test]
fn legal_values_come_from_the_scheme() {
    let (vocab, _) = fixture();
    let dims = vocab
        .legal_values("dimension")
        .expect("dimension enumeration");
    assert!(dims.contains("dim-concept"));
    assert!(!dims.contains("dim-nonexistent"));

    let maturity = vocab
        .legal_values("maturity")
        .expect("maturity enumeration");
    assert_eq!(maturity.len(), 3);
    assert!(maturity.contains("verified"));
}

#[test]
fn group_keys_are_legal_but_not_leaves() {
    // a subject scheme is a taxonomy: binding an attribute to a subject makes
    // the whole subtree available, so a group key is a legal (if coarse) value
    let (vocab, _) = fixture();
    assert!(
        vocab
            .legal_values("dimension")
            .unwrap()
            .contains("dim-common")
    );
    assert!(
        !vocab
            .leaf_values("dimension")
            .unwrap()
            .contains("dim-common")
    );
    assert!(
        vocab
            .leaf_values("dimension")
            .unwrap()
            .contains("dim-install")
    );
}

#[test]
fn bound_subject_itself_is_not_a_value() {
    // @dimension="dimension" is the container's own key, never a valid tag
    let (vocab, _) = fixture();
    assert!(
        !vocab
            .legal_values("dimension")
            .unwrap()
            .contains("dimension")
    );
}

#[test]
fn default_subject_is_read_and_its_absence_is_meaningful() {
    let (vocab, _) = fixture();
    assert_eq!(
        vocab.enumeration("maturity").unwrap().default.as_deref(),
        Some("draft")
    );
    // volatility deliberately has no default: a missing value must be an error,
    // not a silent fallback
    assert_eq!(vocab.enumeration("volatility").unwrap().default, None);
}

#[test]
fn dangling_binding_is_reported_without_losing_the_rest() {
    let (vocab, diag) = fixture();
    assert!(diag.has_errors(), "unknown subject key must be an error");
    assert!(vocab.legal_values("tool").is_none());
    // the other attributes survive
    assert!(vocab.legal_values("dimension").is_some());
    assert!(vocab.legal_values("maturity").is_some());
}

#[test]
fn subject_tree_keeps_hierarchy() {
    let (vocab, _) = fixture();
    let dimension = vocab.subject("dimension").expect("dimension subject");
    assert_eq!(dimension.children.len(), 2);
    let common = vocab.subject("dim-common").expect("dim-common subject");
    assert_eq!(common.children.len(), 2);
}
