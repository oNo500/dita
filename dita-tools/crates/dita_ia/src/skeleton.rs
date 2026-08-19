use crate::{Branches, DomainCoverage, Paint};
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

/// One topic as it appears in the skeleton's leaves: the file that carries it,
/// and how a human should read it.
///
/// `title` is `None` when the topic has no `<title>`/`<glossterm>` at all —
/// parsing already raises a diagnostic warning for that ("topic has no
/// title"), so this is not a second place that judges the omission, only one
/// that has to say plainly it has nothing to show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicRef {
    pub file_name: String,
    pub title: Option<String>,
}

impl TopicRef {
    #[must_use]
    pub fn from_meta(meta: &TopicMeta) -> Self {
        Self {
            file_name: meta
                .path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("?")
                .to_string(),
            title: (!meta.title.is_empty()).then(|| meta.title.clone()),
        }
    }

    /// The label a tree line shows for this topic.
    ///
    /// Title-first — that is the whole point of this change: file names are
    /// ASCII kebab for cross-platform and referencing reasons, not for a
    /// human deciding whether a title is right. `details` appends the file
    /// name after the title (dimmed, parenthesised) rather than replacing it,
    /// matching how `--details` elsewhere adds detail without hiding the
    /// default view. A missing title falls back to the file name — nothing
    /// else to show — marked so the gap itself stays visible instead of
    /// silently reading like a normal, if terse, label.
    #[must_use]
    pub fn label(&self, paint: Paint, details: bool) -> String {
        match &self.title {
            Some(title) if details => {
                format!("{title}  {}", paint.dim(&format!("({})", self.file_name)))
            }
            Some(title) => title.clone(),
            None => format!("{}  {}", self.file_name, paint.red("⚠ 无标题")),
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
    pub topics: Vec<TopicRef>,
    /// Coverage, when a landscape declares this domain.
    pub coverage: Option<(usize, usize)>,
    pub children: Vec<Node>,
    /// Topics under this branch that declared no domain, so nothing places them
    /// under a sub-topic. Only branches collect these.
    pub unplaced: Vec<TopicRef>,
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
    let mut by_domain: BTreeMap<&str, Vec<TopicRef>> = BTreeMap::new();
    for meta in input.topics {
        if let Some(domain) = &meta.domain {
            by_domain
                .entry(domain.as_str())
                .or_default()
                .push(TopicRef::from_meta(meta));
        }
    }
    let coverage: BTreeMap<&str, &DomainCoverage> = input
        .coverage
        .iter()
        .map(|c| (c.domain.as_str(), c))
        .collect();

    let valid_keys: BTreeSet<String> = subject.all_keys().into_iter().collect();
    let mut nodes: Vec<Node> = subject
        .children
        .iter()
        .map(|branch| {
            let mut node = build_node(branch, &by_domain, &coverage);
            node.benchmark = input.benchmarks.get(&branch.keys).cloned();
            attach_unplaced(&mut node, branch, input, &valid_keys);
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
                .map(TopicRef::from_meta)
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
    by_domain: &BTreeMap<&str, Vec<TopicRef>>,
    coverage: &BTreeMap<&str, &DomainCoverage>,
) -> Node {
    let topics = by_domain
        .get(subject.keys.as_str())
        .cloned()
        .unwrap_or_default();
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
fn attach_unplaced(
    node: &mut Node,
    subject: &Subject,
    input: &Input,
    valid_keys: &BTreeSet<String>,
) {
    let Some((label, paths)) = branch_topics(subject, input) else {
        return;
    };
    // Placed means the declared domain actually names a subject key. A typo'd
    // domain must fall back into this bucket: treating "declared anything" as
    // placed makes the topic hang nowhere and vanish from the skeleton — worse
    // than undeclared, which at least stays visible here.
    let placed: BTreeSet<&str> = input
        .topics
        .iter()
        .filter(|t| t.domain.as_ref().is_some_and(|d| valid_keys.contains(d)))
        .map(|t| t.path.to_str().unwrap_or_default())
        .collect();
    node.label.get_or_insert(label);
    for path in paths {
        if !placed.contains(path.to_str().unwrap_or_default()) {
            if let Some(meta) = input.topics.iter().find(|t| &t.path == path) {
                node.unplaced.push(TopicRef::from_meta(meta));
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
        (path.file_stem().and_then(|s| s.to_str()) == Some(subject.keys.as_str())).then_some(label)
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
    let children_done =
        !node.children.is_empty() && node.children.iter().all(|c| c.state == State::Done);
    (own || children_done) && node.unplaced.is_empty()
}

#[cfg(test)]
mod tests {
    use super::{Paint, TopicMeta, TopicRef};
    use dita_ast::TopicType;
    use std::path::PathBuf;

    fn meta(title: &str) -> TopicMeta {
        TopicMeta {
            path: PathBuf::from("topics/demo/agent-context-budget.dita"),
            id: None,
            title: title.to_string(),
            topic_type: TopicType::Concept,
            lang: None,
            maturity: None,
            volatility: None,
            dimensions: Vec::new(),
            tools: Vec::new(),
            domain: None,
            planned_dimensions: Vec::new(),
            reviewed: None,
        }
    }

    #[test]
    fn label_prefers_the_title_over_the_file_name() {
        let t = TopicRef::from_meta(&meta("上下文预算"));
        assert_eq!(t.label(Paint::off(), false), "上下文预算");
    }

    #[test]
    fn details_appends_the_file_name_after_the_title() {
        let t = TopicRef::from_meta(&meta("上下文预算"));
        assert_eq!(
            t.label(Paint::off(), true),
            "上下文预算  (agent-context-budget.dita)"
        );
    }

    #[test]
    fn a_missing_title_falls_back_to_the_file_name_and_is_marked() {
        let t = TopicRef::from_meta(&meta(""));
        assert_eq!(t.title, None);
        let label = t.label(Paint::off(), false);
        assert!(label.contains("agent-context-budget.dita"));
        assert!(label.contains("无标题"), "must flag the gap: {label}");
    }
}
