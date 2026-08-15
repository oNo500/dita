use dita_ast::{
    DitaMap, TopicRef,
    visit::{Visit, walk_dita_map},
};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// Find all `.dita` files under `topics_root` that no map references.
///
/// Every map given is consulted, not just the root: a topic reachable only from
/// a deliverable map is referenced, not orphaned. Judging orphanhood from the
/// root map alone reports false positives for exactly those topics.
pub fn find_orphans(maps: &[DitaMap], topics_root: &Path) -> Vec<PathBuf> {
    let mut referenced = HashSet::new();
    for map in maps {
        referenced.extend(collect_referenced(map));
    }
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

/// Collect every `.ditamap` under `dir`, sorted. Used to consult all maps —
/// domain, glossary and deliverable — when deciding what counts as an orphan.
pub fn find_maps(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                result.extend(find_maps(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("ditamap") {
                result.push(path);
            }
        }
    }
    result.sort();
    result
}
