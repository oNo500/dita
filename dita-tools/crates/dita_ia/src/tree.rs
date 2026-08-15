use dita_ast::{DitaMap, MapNode};
use std::path::Path;

pub fn print_tree(map: &DitaMap, topics_root: &Path) {
    println!("{} (root)", map.title);
    print_nodes(&map.children, "", topics_root);
}

fn print_nodes(nodes: &[MapNode], prefix: &str, topics_root: &Path) {
    let count = nodes.len();
    for (i, node) in nodes.iter().enumerate() {
        let is_last = i == count - 1;
        let conn = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };

        match node {
            MapNode::TopicRef(t) => {
                let name = t
                    .href
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("?");
                let marker = if t.href.exists() { "✓" } else { "✗" };
                println!("{prefix}{conn}{marker} {name}");
            }
            MapNode::TopicHead(h) => {
                let n = count_topics(&h.children);
                let label = if n == 0 {
                    format!("[空] {}", h.nav_title)
                } else {
                    format!("[{n}]  {}", h.nav_title)
                };
                println!("{prefix}{conn}{label}");
                print_nodes(&h.children, &child_prefix, topics_root);
            }
            MapNode::MapRef(_) => {} // resource-only, skip from display
        }
    }
}

pub fn count_topics(nodes: &[MapNode]) -> usize {
    nodes
        .iter()
        .map(|n| match n {
            MapNode::TopicRef(_) => 1,
            MapNode::TopicHead(h) => count_topics(&h.children),
            MapNode::MapRef(_) => 0,
        })
        .sum()
}
