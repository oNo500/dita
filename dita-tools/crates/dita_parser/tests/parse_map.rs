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
fn keeps_mapref_as_its_own_node() {
    let fixture = Path::new("tests/fixtures/simple.ditamap");
    let (map, _diag) = parse_map(fixture).expect("parse failed");
    // the mapref to sub.ditamap + simple.ditamap's own topicref = 2 nodes;
    // the sub map is resolved but not spliced into the parent
    assert_eq!(map.children.len(), 2);
    let MapNode::MapRef(sub) = &map.children[0] else {
        panic!("expected the mapref to survive as its own node");
    };
    assert_eq!(sub.title.as_deref(), Some("子 Map"));
    assert_eq!(sub.children.len(), 1);
    assert!(matches!(sub.children[0], MapNode::TopicRef(_)));
    assert!(matches!(map.children[1], MapNode::TopicRef(_)));
}

#[test]
fn empty_submap_stays_visible() {
    // the defect this guards: an inlined childless map vanishes entirely, so an
    // IA view cannot report "this domain exists and is empty"
    let fixture = Path::new("tests/fixtures/with-empty.ditamap");
    let (map, _diag) = parse_map(fixture).expect("parse failed");
    assert_eq!(map.children.len(), 1);
    let MapNode::MapRef(sub) = &map.children[0] else {
        panic!("expected an empty mapref node");
    };
    assert_eq!(sub.title.as_deref(), Some("空领域"));
    assert!(sub.children.is_empty());
}

#[test]
fn diamond_reference_is_not_a_cycle() {
    // the same map reached twice through different parents is legal DITA;
    // only a map referencing one of its own ancestors is a cycle
    let fixture = Path::new("tests/fixtures/diamond.ditamap");
    let (map, diag) = parse_map(fixture).expect("parse failed");
    assert_eq!(map.children.len(), 2);
    assert!(
        !diag.items.iter().any(|d| d.message().contains("circular")),
        "diamond reference must not be reported as circular"
    );
}

#[test]
fn reports_missing_topic_files() {
    let fixture = Path::new("tests/fixtures/simple.ditamap");
    let (_map, diag) = parse_map(fixture).expect("parse failed");
    // foo.dita and bar.dita do not exist on disk → errors
    assert!(diag.has_errors());
    assert_eq!(diag.error_count(), 2);
}
