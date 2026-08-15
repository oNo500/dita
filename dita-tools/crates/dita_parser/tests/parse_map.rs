use dita_ast::MapNode;
use dita_parser::parse_map;
use std::path::Path;

#[test]
fn parses_title_and_lang() {
    let fixture = Path::new("tests/fixtures/simple.ditamap");
    let (map, _diag) = parse_map(fixture).expect("parse failed");
    assert_eq!(map.title, "测试知识体系");
    assert_eq!(map.lang.as_deref(), Some("zh-CN"));
}

#[test]
fn expands_mapref_inline() {
    let fixture = Path::new("tests/fixtures/simple.ditamap");
    let (map, _diag) = parse_map(fixture).expect("parse failed");
    // sub.ditamap's topicref (bar.dita) + simple.ditamap's topicref (foo.dita) = 2 nodes
    assert_eq!(map.children.len(), 2);
    assert!(map.children.iter().all(|n| matches!(n, MapNode::TopicRef(_))));
}

#[test]
fn reports_missing_topic_files() {
    let fixture = Path::new("tests/fixtures/simple.ditamap");
    let (_map, diag) = parse_map(fixture).expect("parse failed");
    // foo.dita and bar.dita do not exist on disk → errors
    assert!(diag.has_errors());
    assert_eq!(diag.error_count(), 2);
}
