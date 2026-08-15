use dita_ast::{visit::{Visit, walk_dita_map}, DitaMap, TopicRef};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// Find all `.dita` files under `topics_root` that are not referenced
/// by any topicref in the map tree. These are "orphan topics".
pub fn find_orphans(map: &DitaMap, topics_root: &Path) -> Vec<PathBuf> {
    let referenced = collect_referenced(map);
    let mut all = walkdir(topics_root);
    all.retain(|p| !referenced.contains(p));
    all
}

fn collect_referenced(map: &DitaMap) -> HashSet<PathBuf> {
    struct Collector(HashSet<PathBuf>);

    impl Visit for Collector {
        fn visit_topic_ref(&mut self, node: &TopicRef) {
            if let Ok(canonical) = node.href.canonicalize() {
                self.0.insert(canonical);
            }
        }
    }

    let mut collector = Collector(HashSet::new());
    walk_dita_map(&mut collector, map);
    collector.0
}

fn walkdir(root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                result.extend(walkdir(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("dita") {
                if let Ok(canonical) = path.canonicalize() {
                    result.push(canonical);
                }
            }
        }
    }
    result.sort();
    result
}
