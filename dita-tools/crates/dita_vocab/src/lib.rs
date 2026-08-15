//! Reads a DITA subject scheme map: the controlled vocabulary.
//!
//! Legal values live in the subject scheme and nowhere else. This crate exists
//! so that no Rust source has to carry a copy of them — a hand-kept second copy
//! is exactly the defect this toolchain is meant to remove (see
//! `docs/架构与边界.md`, 唯一事实源清单).

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    path::Path,
};

use anyhow::Context;
use dita_diagnostics::{Diagnostic, DiagnosticBag};

/// One `<subjectdef>` node and the subtree below it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subject {
    pub keys: String,
    pub nav_title: Option<String>,
    /// `<data>` children, by `@name`. The value is `@value` when present and
    /// the element text otherwise — the benchmark registry uses both forms
    /// (`last-benchmarked` carries an attribute, `anchor` carries prose).
    pub data: BTreeMap<String, String>,
    pub children: Vec<Subject>,
}

impl Subject {
    /// This subject's key plus every descendant key.
    ///
    /// A subject scheme is a taxonomy: binding an attribute to a subject makes
    /// the whole subtree available, not only the leaves. Tagging with a group
    /// key (`dim-common`) is therefore legal, if coarse — callers that care
    /// about the distinction can ask with [`Subject::leaf_keys`].
    #[must_use]
    pub fn all_keys(&self) -> HashSet<String> {
        let mut out = HashSet::new();
        self.collect_keys(&mut out);
        out
    }

    fn collect_keys(&self, out: &mut HashSet<String>) {
        out.insert(self.keys.clone());
        for child in &self.children {
            child.collect_keys(out);
        }
    }

    /// Keys of subjects with no children — the most specific values available.
    #[must_use]
    pub fn leaf_keys(&self) -> HashSet<String> {
        let mut out = HashSet::new();
        self.collect_leaves(&mut out);
        out
    }

    fn collect_leaves(&self, out: &mut HashSet<String>) {
        if self.children.is_empty() {
            out.insert(self.keys.clone());
        }
        for child in &self.children {
            child.collect_leaves(out);
        }
    }
}

/// One `<enumerationdef>`: an attribute bound to a subject subtree.
#[derive(Debug, Clone)]
pub struct Enumeration {
    pub attribute: String,
    /// Key of the subject the attribute is bound to.
    pub subject_key: String,
    /// `<defaultSubject>`, when the scheme declares one. Its absence is
    /// meaningful: this vocabulary deliberately gives `volatility` no default
    /// so that a missing value is an error rather than a silent fallback.
    pub default: Option<String>,
    pub values: HashSet<String>,
    pub leaf_values: HashSet<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Vocabulary {
    /// Top-level subjects, in document order.
    pub subjects: Vec<Subject>,
    by_key: HashMap<String, Subject>,
    enums: HashMap<String, Enumeration>,
}

impl Vocabulary {
    /// Legal values for an attribute, or `None` if the scheme binds no
    /// enumeration to it.
    #[must_use]
    pub fn legal_values(&self, attribute: &str) -> Option<&HashSet<String>> {
        self.enums.get(attribute).map(|e| &e.values)
    }

    /// Legal values that are leaves of the taxonomy.
    #[must_use]
    pub fn leaf_values(&self, attribute: &str) -> Option<&HashSet<String>> {
        self.enums.get(attribute).map(|e| &e.leaf_values)
    }

    #[must_use]
    pub fn enumeration(&self, attribute: &str) -> Option<&Enumeration> {
        self.enums.get(attribute)
    }

    #[must_use]
    pub fn subject(&self, key: &str) -> Option<&Subject> {
        self.by_key.get(key)
    }

    pub fn attributes(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.enums.keys().map(String::as_str).collect();
        names.sort_unstable();
        names
    }
}

/// Parse a `<subjectScheme>` map.
///
/// Problems *inside* the scheme are reported as diagnostics rather than errors:
/// a vocabulary with one dangling `keyref` is still useful for every other
/// attribute, and refusing to read it would take the whole IA view down.
///
/// # Errors
///
/// Returns `Err` only when the file itself cannot be used: the path does not
/// resolve, the file cannot be read, or it is not well-formed XML.
pub fn parse_vocab(path: &Path) -> anyhow::Result<(Vocabulary, DiagnosticBag)> {
    let mut diag = DiagnosticBag::default();
    let canonical = path
        .canonicalize()
        .with_context(|| format!("cannot resolve path: {}", path.display()))?;
    let xml = fs::read_to_string(&canonical)
        .with_context(|| format!("cannot read file: {}", canonical.display()))?;
    // DITA files carry a <!DOCTYPE ...>, so the parser has to allow a DTD
    let opts = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };
    let doc = roxmltree::Document::parse_with_options(&xml, opts)
        .with_context(|| format!("XML parse error in: {}", canonical.display()))?;

    let root = doc.root_element();
    let mut subjects = Vec::new();
    let mut by_key = HashMap::new();
    for child in root.children().filter(roxmltree::Node::is_element) {
        if child.tag_name().name() == "subjectdef" {
            if let Some(subject) = read_subject(child, &canonical, &mut diag) {
                index(&subject, &mut by_key);
                subjects.push(subject);
            }
        }
    }

    let mut enums = HashMap::new();
    for node in root.children().filter(roxmltree::Node::is_element) {
        if node.tag_name().name() != "enumerationdef" {
            continue;
        }
        let Some(attribute) = child_attr(node, "attributedef", "name") else {
            diag.push(Diagnostic::warning(
                &canonical,
                "enumerationdef without attributedef/@name — skipped",
            ));
            continue;
        };
        let Some(subject_key) = child_attr(node, "subjectdef", "keyref") else {
            diag.push(Diagnostic::warning(
                &canonical,
                format!("enumerationdef for @{attribute} binds no subjectdef/@keyref — skipped"),
            ));
            continue;
        };
        let Some(subject) = by_key.get(&subject_key) else {
            diag.push(Diagnostic::error(
                &canonical,
                format!("@{attribute} is bound to unknown subject key \"{subject_key}\""),
            ));
            continue;
        };
        enums.insert(
            attribute.clone(),
            Enumeration {
                attribute,
                subject_key,
                default: child_attr(node, "defaultSubject", "keyref"),
                // the bound subject itself is a container, not a value
                values: subject
                    .children
                    .iter()
                    .flat_map(Subject::all_keys)
                    .collect(),
                leaf_values: subject
                    .children
                    .iter()
                    .flat_map(Subject::leaf_keys)
                    .collect(),
            },
        );
    }

    Ok((
        Vocabulary {
            subjects,
            by_key,
            enums,
        },
        diag,
    ))
}

fn read_subject(
    node: roxmltree::Node,
    source: &Path,
    diag: &mut DiagnosticBag,
) -> Option<Subject> {
    let Some(keys) = node.attribute("keys") else {
        // a keyref-only subjectdef is a reference, not a definition
        if node.attribute("keyref").is_none() {
            diag.push(Diagnostic::warning(
                source,
                "subjectdef without @keys or @keyref — skipped",
            ));
        }
        return None;
    };
    let children = node
        .children()
        .filter(roxmltree::Node::is_element)
        .filter(|n| n.tag_name().name() == "subjectdef")
        .filter_map(|n| read_subject(n, source, diag))
        .collect();
    Some(Subject {
        keys: keys.to_string(),
        nav_title: nav_title(node),
        data: read_data(node),
        children,
    })
}

/// Direct `<data>` children only: a nested subject's data belongs to it, not
/// to its parent.
fn read_data(node: roxmltree::Node) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for data in node
        .children()
        .filter(roxmltree::Node::is_element)
        .filter(|n| n.tag_name().name() == "data")
    {
        let Some(name) = data.attribute("name") else {
            continue;
        };
        let value = data.attribute("value").map(str::to_string).or_else(|| {
            let text: String = data
                .descendants()
                .filter(roxmltree::Node::is_text)
                .filter_map(|n| n.text())
                .collect();
            let trimmed = text.trim().to_string();
            (!trimmed.is_empty()).then_some(trimmed)
        });
        if let Some(value) = value {
            out.insert(name.to_string(), value);
        }
    }
    out
}

fn nav_title(node: roxmltree::Node) -> Option<String> {
    node.children()
        .find(|n| n.has_tag_name("topicmeta"))?
        .children()
        .find(|n| n.has_tag_name("navtitle"))?
        .text()
        .map(str::to_string)
}

fn child_attr(node: roxmltree::Node, tag: &str, attr: &str) -> Option<String> {
    node.children()
        .filter(roxmltree::Node::is_element)
        .find(|n| n.tag_name().name() == tag)?
        .attribute(attr)
        .map(str::to_string)
}

fn index(subject: &Subject, out: &mut HashMap<String, Subject>) {
    out.insert(subject.keys.clone(), subject.clone());
    for child in &subject.children {
        index(child, out);
    }
}
