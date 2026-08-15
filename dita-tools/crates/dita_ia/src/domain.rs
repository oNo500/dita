use dita_ast::{DitaMap, MapNode};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

/// Which top-level branch a topic hangs under.
///
/// A branch is a child of the root map — in this library one of the nine
/// domain maps, wrapped in a `topichead` so it keeps a navigation node.
///
/// This is *not* the same thing as the `domain` a topic declares in its prolog.
/// Branch membership is structural and coarse (`web`); a declared domain is a
/// technology domain (`electron`), several of which can live under one branch,
/// each with its own planned dimensions. Coverage is measured per technology
/// domain, statistics per branch. Deriving one from the other would merge
/// unrelated plans.
#[derive(Debug, Default)]
pub struct Branches {
    /// branch label → topic paths, in tree order
    pub topics: BTreeMap<String, Vec<PathBuf>>,
    branch_of: BTreeMap<PathBuf, Vec<String>>,
}

impl Branches {
    /// Branch labels a topic hangs under. More than one means it is
    /// cross-referenced, which this library allows.
    pub fn of(&self, topic: &Path) -> &[String] {
        self.branch_of.get(topic).map_or(&[], Vec::as_slice)
    }

    #[must_use]
    pub fn empty_branches(&self) -> Vec<&str> {
        self.topics
            .iter()
            .filter(|(_, t)| t.is_empty())
            .map(|(name, _)| name.as_str())
            .collect()
    }
}

/// Read branch membership off the root map's own shape.
#[must_use]
pub fn branches(map: &DitaMap) -> Branches {
    let mut out = Branches::default();
    for node in &map.children {
        let Some(label) = branch_label(node) else {
            continue;
        };
        let mut found = Vec::new();
        collect_topics(std::slice::from_ref(node), &mut found);
        for path in &found {
            out.branch_of
                .entry(path.clone())
                .or_default()
                .push(label.clone());
        }
        out.topics.entry(label).or_default().extend(found);
    }
    out
}

/// The label of a top-level node, or `None` for nodes that are not branches
/// (resource-only maps carry no navigation and no content).
fn branch_label(node: &MapNode) -> Option<String> {
    match node {
        MapNode::TopicHead(h) => Some(h.nav_title.clone()),
        MapNode::MapRef(m) => {
            if m.processing_role == dita_ast::ProcessingRole::ResourceOnly {
                return None;
            }
            m.title.clone().or_else(|| {
                m.href
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(str::to_string)
            })
        }
        // a topic sitting directly at root belongs to no branch
        MapNode::TopicRef(_) => None,
    }
}

fn collect_topics(nodes: &[MapNode], out: &mut Vec<PathBuf>) {
    for node in nodes {
        match node {
            MapNode::TopicRef(t) => {
                let path = t.href.canonicalize().unwrap_or_else(|_| t.href.clone());
                out.push(path);
            }
            MapNode::TopicHead(h) => collect_topics(&h.children, out),
            MapNode::MapRef(m) => collect_topics(&m.children, out),
        }
    }
}
