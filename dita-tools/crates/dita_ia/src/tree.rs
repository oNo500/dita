use dita_ast::{DitaMap, MapNode, ProcessingRole};

pub fn print_tree(map: &DitaMap) {
    println!("{} (root)", map.title);
    print_nodes(&map.children, "");
}

fn print_nodes(nodes: &[MapNode], prefix: &str) {
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
                let name = t.href.file_name().and_then(|s| s.to_str()).unwrap_or("?");
                let marker = if t.href.exists() { "✓" } else { "✗" };
                println!("{prefix}{conn}{marker} {name}");
            }
            MapNode::TopicHead(h) => {
                println!("{prefix}{conn}{} {}", count_label(&h.children), h.nav_title);
                // A topichead wrapping a single same-named mapref exists only to
                // give the referenced map a navigation node (merge semantics give
                // it none). Printing both would show one branch twice; the titles
                // are kept in step by consistency::check_group_titles, and a
                // mismatch still prints both plus a warning.
                if let [MapNode::MapRef(m)] = h.children.as_slice() {
                    if m.title.as_deref() == Some(h.nav_title.as_str()) {
                        print_nodes(&m.children, &child_prefix);
                        continue;
                    }
                }
                print_nodes(&h.children, &child_prefix);
            }
            MapNode::MapRef(m) => {
                // resource-only maps (subject schemes, key definitions) carry no
                // navigation — listing them as empty branches would be misleading
                if m.processing_role == ProcessingRole::ResourceOnly {
                    let name = m.href.file_name().and_then(|s| s.to_str()).unwrap_or("?");
                    println!("{prefix}{conn}◦ {name}（resource-only，不进导航）");
                    continue;
                }
                let label = m.title.clone().unwrap_or_else(|| {
                    m.href
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("?")
                        .to_string()
                });
                println!("{prefix}{conn}{} {label}", count_label(&m.children));
                print_nodes(&m.children, &child_prefix);
            }
        }
    }
}

/// `[空]` is the whole point of the IA view: a domain that exists but holds
/// nothing has to be visible, not silently absent.
fn count_label(nodes: &[MapNode]) -> String {
    let n = count_topics(nodes);
    if n == 0 {
        "[空]".to_string()
    } else {
        format!("[{n}]")
    }
}

pub fn count_topics(nodes: &[MapNode]) -> usize {
    nodes
        .iter()
        .map(|n| match n {
            MapNode::TopicRef(_) => 1,
            MapNode::TopicHead(h) => count_topics(&h.children),
            MapNode::MapRef(m) => count_topics(&m.children),
        })
        .sum()
}
