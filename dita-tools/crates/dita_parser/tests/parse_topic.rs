use dita_ast::TopicType;
use dita_parser::parse_topic;
use std::path::{Path, PathBuf};

fn fixture(name: &str) -> PathBuf {
    Path::new("tests/fixtures/topics").join(name)
}

#[test]
fn reads_every_declared_field() {
    let (meta, diag) = parse_topic(&fixture("full.dita")).expect("parse failed");
    assert_eq!(meta.topic_type, TopicType::Concept);
    assert_eq!(meta.id.as_deref(), Some("full-topic"));
    assert_eq!(meta.lang.as_deref(), Some("zh-CN"));
    assert_eq!(meta.maturity.as_deref(), Some("curated"));
    assert_eq!(meta.volatility.as_deref(), Some("volatile"));
    assert_eq!(meta.dimensions, ["dim-mechanism", "dim-decision"]);
    assert_eq!(meta.domain.as_deref(), Some("web"));
    assert_eq!(meta.reviewed.as_deref(), Some("2026-08-15"));
    assert!(!diag.has_errors());
}

#[test]
fn title_includes_text_inside_markup() {
    // titles carry <term>, <xmlelement> and friends; taking only the direct
    // text node would silently truncate them
    let (meta, _) = parse_topic(&fixture("full.dita")).expect("parse failed");
    assert_eq!(meta.title, "标题里带标记的情形");
}

#[test]
fn reads_planned_dimensions_from_a_landscape() {
    let (meta, _) = parse_topic(&fixture("landscape.dita")).expect("parse failed");
    assert_eq!(
        meta.planned_dimensions,
        ["dim-concept", "dim-usage", "dim-security"]
    );
    assert_eq!(meta.domain.as_deref(), Some("demo"));
    assert!(
        meta.dimensions.is_empty(),
        "a landscape plans, it does not cover"
    );
}

#[test]
fn missing_attributes_are_none_not_defaults() {
    // the parser must not substitute the scheme's defaultSubject: "forgot to
    // tag" and "chose the default" have to stay distinguishable, which is what
    // R2 relies on
    let (meta, _) = parse_topic(&fixture("bare.dita")).expect("parse failed");
    assert_eq!(meta.topic_type, TopicType::Reference);
    assert_eq!(meta.maturity, None);
    assert_eq!(meta.volatility, None);
    assert!(meta.dimensions.is_empty());
    assert_eq!(meta.domain, None);
}

#[test]
fn glossentry_title_comes_from_glossterm() {
    let (meta, diag) = parse_topic(&fixture("glossary-term.dita")).expect("parse failed");
    assert_eq!(meta.topic_type, TopicType::GlossEntry);
    assert_eq!(meta.title, "示例术语");
    assert!(!diag.has_errors());
}

#[test]
fn illegal_dimension_values_are_reported_verbatim() {
    // validation belongs to the layer that owns the vocabulary; the parser only
    // has to hand the value over unchanged
    let (meta, _) = parse_topic(&fixture("illegal-dimension.dita")).expect("parse failed");
    assert_eq!(meta.dimensions, ["dim-concept", "dim-nonexistent"]);
}

#[test]
fn unknown_root_element_warns_without_failing() {
    let (meta, diag) = parse_topic(&fixture("not-a-topic.dita")).expect("parse failed");
    assert_eq!(meta.topic_type, TopicType::Unknown);
    assert_eq!(diag.warning_count(), 1);
    assert!(!diag.has_errors());
}
