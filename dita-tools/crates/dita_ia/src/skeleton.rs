use crate::{Branches, DomainCoverage};
use dita_ast::TopicMeta;
use dita_vocab::{Subject, Vocabulary};
use std::collections::{BTreeMap, BTreeSet};

/// Node state. Four states, one of which a node is always in.
///
/// See docs/plans/2026-08-15-skeleton-design.md: the symbols carry the meaning
/// on their own, colour only speeds up scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Planned by the vocabulary, nothing written.
    Unbuilt,
    /// Has content, has not met the completion test.
    InProgress,
    /// Has a landscape (R9) and no blind spots in the dimensions it declares.
    Done,
    /// Organisational node with no vocabulary key — completion does not apply.
    NotApplicable,
}

impl State {
    #[must_use]
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Unbuilt => "○",
            Self::InProgress => "◐",
            Self::Done => "●",
            Self::NotApplicable => "·",
        }
    }
}

/// One node of the skeleton: a subject key, what hangs under it, and how it is
/// doing.
#[derive(Debug)]
pub struct Node {
    pub key: String,
    pub label: Option<String>,
    pub state: State,
    /// Topics that declared this key as their domain.
    pub topics: Vec<String>,
    /// Coverage, when a landscape declares this domain.
    pub coverage: Option<(usize, usize)>,
    pub children: Vec<Node>,
    /// Topics under this branch that declared no domain, so nothing places them
    /// under a sub-topic. Only branches collect these.
    pub unplaced: Vec<String>,
    /// Map groups under this branch with no corresponding vocabulary key.
    pub outside: Vec<String>,
    pub benchmark: Option<String>,
}

impl Node {
    /// Topics anywhere in this subtree, plus whatever could not be placed.
    #[must_use]
    pub fn total_topics(&self) -> usize {
        self.topics.len()
            + self.unplaced.len()
            + self.children.iter().map(Node::total_topics).sum::<usize>()
    }

    /// Direct children that hold something.
    #[must_use]
    pub fn built_children(&self) -> usize {
        self.children
            .iter()
            .filter(|c| c.total_topics() > 0)
            .count()
    }
}

pub struct Input<'a> {
    pub vocab: &'a Vocabulary,
    pub topics: &'a [TopicMeta],
    pub branches: &'a Branches,
    pub coverage: &'a [DomainCoverage],
    pub benchmarks: &'a BTreeMap<String, String>,
}

/// Build the skeleton from the subject tree.
#[must_use]
pub fn build(input: &Input) -> Vec<Node> {
    let Some(subject) = input.vocab.subject("subject") else {
        return Vec::new();
    };

    // topics that named a subject key as their domain
    let mut by_domain: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for meta in input.topics {
        if let Some(domain) = &meta.domain {
            by_domain
                .entry(domain.as_str())
                .or_default()
                .push(file_name(meta));
        }
    }
    let coverage: BTreeMap<&str, &DomainCoverage> =
        input.coverage.iter().map(|c| (c.domain.as_str(), c)).collect();

    let mut nodes: Vec<Node> = subject
        .children
        .iter()
        .map(|branch| {
            let mut node = build_node(branch, &by_domain, &coverage);
            node.benchmark = input.benchmarks.get(&branch.keys).cloned();
            attach_unplaced(&mut node, branch, input);
            recompute_state(&mut node);
            node
        })
        .collect();

    // Branches the maps have but the vocabulary does not key — the glossary is
    // organisational, not a subject. Leaving them out would make the skeleton
    // claim the library holds less than it does.
    let keyed: BTreeSet<&str> = subject.children.iter().map(|c| c.keys.as_str()).collect();
    for (label, paths) in &input.branches.topics {
        let stem = input
            .branches
            .source_map
            .get(label)
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if keyed.contains(stem) {
            continue;
        }
        nodes.push(Node {
            key: stem.to_string(),
            label: Some(label.clone()),
            state: State::NotApplicable,
            topics: paths
                .iter()
                .filter_map(|p| input.topics.iter().find(|t| &t.path == p))
                .map(file_name)
                .collect(),
            coverage: None,
            children: Vec::new(),
            unplaced: Vec::new(),
            outside: Vec::new(),
            benchmark: None,
        });
    }
    nodes
}

fn build_node(
    subject: &Subject,
    by_domain: &BTreeMap<&str, Vec<String>>,
    coverage: &BTreeMap<&str, &DomainCoverage>,
) -> Node {
    let topics = by_domain.get(subject.keys.as_str()).cloned().unwrap_or_default();
    let cov = coverage
        .get(subject.keys.as_str())
        .map(|c| (c.covered.len(), c.planned.len()));
    Node {
        key: subject.keys.clone(),
        label: subject.nav_title.clone(),
        state: State::Unbuilt,
        topics,
        coverage: cov,
        children: subject
            .children
            .iter()
            .map(|c| build_node(c, by_domain, coverage))
            .collect(),
        unplaced: Vec::new(),
        outside: Vec::new(),
        benchmark: None,
    }
}

/// Collect what the maps hold for this branch but the vocabulary cannot place.
///
/// A topic reaches a sub-topic node only by declaring `domain`; without that
/// declaration the vocabulary tree has no idea where it belongs. Bucketing them
/// visibly is the point — it is currently the largest structural gap in the
/// library, and hiding it would hide exactly what this view exists to show.
fn attach_unplaced(node: &mut Node, subject: &Subject, input: &Input) {
    let Some((label, paths)) = branch_topics(subject, input) else {
        return;
    };
    let placed: BTreeSet<&str> = input
        .topics
        .iter()
        .filter(|t| t.domain.is_some())
        .map(|t| t.path.to_str().unwrap_or_default())
        .collect();
    node.label.get_or_insert(label);
    for path in paths {
        if !placed.contains(path.to_str().unwrap_or_default()) {
            if let Some(meta) = input.topics.iter().find(|t| &t.path == path) {
                node.unplaced.push(file_name(meta));
            }
        }
    }
}

/// The branch label and topic paths for a subject key, matched through the
/// branch map's file name (`domains/web.ditamap` ↔ key `web`).
fn branch_topics<'a>(
    subject: &Subject,
    input: &'a Input,
) -> Option<(String, &'a Vec<std::path::PathBuf>)> {
    let label = input.branches.source_map.iter().find_map(|(label, path)| {
        (path.file_stem().and_then(|s| s.to_str()) == Some(subject.keys.as_str()))
            .then_some(label)
    })?;
    Some((label.clone(), input.branches.topics.get(label)?))
}

/// Done means: a landscape exists and every dimension it planned is covered.
/// Anything holding content but short of that is in progress.
fn recompute_state(node: &mut Node) {
    for child in &mut node.children {
        recompute_state(child);
    }
    node.state = if node.total_topics() == 0 {
        State::Unbuilt
    } else if is_done(node) {
        State::Done
    } else {
        State::InProgress
    };
}

fn is_done(node: &Node) -> bool {
    // Its own landscape covers every dimension it planned...
    let own = node
        .coverage
        .is_some_and(|(covered, planned)| planned > 0 && covered == planned);
    // ...or every sub-topic it plans is itself done. An empty child is *not*
    // done — treating "all children empty" as satisfied would mark a branch
    // complete precisely when nothing under it had been written.
    let children_done = !node.children.is_empty()
        && node.children.iter().all(|c| c.state == State::Done);
    (own || children_done) && node.unplaced.is_empty()
}

fn file_name(meta: &TopicMeta) -> String {
    meta.path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("?")
        .to_string()
}
