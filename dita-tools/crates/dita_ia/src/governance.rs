use crate::domain::Branches;
use crate::stats::BranchStats;
use dita_ast::TopicMeta;
use dita_vocab::Vocabulary;
use std::collections::{BTreeMap, BTreeSet};

/// What the vocabulary plans for a branch versus what the maps actually hold.
///
/// The subject tree is the "ought": it names every sub-topic the taxonomy
/// intends to carry. Without it the tool can only say "this branch has three
/// topics"; with it, "this branch plans eighteen sub-topics and has built
/// none".
#[derive(Debug)]
pub struct BranchPlan {
    pub key: String,
    /// Direct sub-topics named in the subject tree.
    pub planned: Vec<String>,
    /// Every descendant of the branch key, not only direct children.
    pub planned_total: usize,
    /// Topics actually present in the branch this key maps to, if matched.
    pub built: usize,
    /// The branch label the key was matched to, if any.
    pub matched_branch: Option<String>,
}

/// A benchmark-registry entry: when this branch of the taxonomy was last
/// checked against outside reference points, and how often it should be.
///
/// This is the only data in the library with a decay clock on it. Nothing was
/// reading it, which is the same failure the volatility attribute was created
/// to prevent — relying on someone remembering.
#[derive(Debug)]
pub struct BenchmarkEntry {
    pub key: String,
    pub anchor: Option<String>,
    pub last_benchmarked: Option<String>,
    pub cadence: Option<String>,
}

impl BenchmarkEntry {
    /// Months after which this entry is due, from its cadence. `None` means the
    /// registry says event-triggered only — no calendar expiry to compute.
    #[must_use]
    pub fn due_months(&self) -> Option<u32> {
        match self.cadence.as_deref()? {
            c if c.contains("6mo") => Some(6),
            c if c.contains("annual") => Some(12),
            _ => None,
        }
    }
}

/// Controlled values that exist in the vocabulary but that nothing uses.
///
/// An unused value is either premature planning or dead weight; either way the
/// vocabulary claims a distinction the content does not make.
#[derive(Debug)]
pub struct ValueUsage {
    pub attribute: String,
    pub used: BTreeMap<String, usize>,
    pub unused: BTreeSet<String>,
}

#[must_use]
pub fn branch_plans(
    vocab: &Vocabulary,
    branches: &Branches,
    stats: &[BranchStats],
) -> Vec<BranchPlan> {
    let Some(subject) = vocab.subject("subject") else {
        return Vec::new();
    };
    subject
        .children
        .iter()
        .map(|branch| {
            let matched = match_branch(&branch.keys, branches, stats);
            BranchPlan {
                key: branch.keys.clone(),
                planned: branch.children.iter().map(|c| c.keys.clone()).collect(),
                planned_total: branch.all_keys().len() - 1,
                built: matched.map_or(0, |s| s.topics),
                matched_branch: matched.map(|s| s.name.clone()),
            }
        })
        .collect()
}

/// Match a subject key to a branch in the maps by the branch map's file name.
///
/// The scheme keys branches in English (`web`) and the maps title them in
/// Chinese (`Web 技术栈`); nothing declares the correspondence. The file name
/// does carry it — `domains/web.ditamap` — so that is what is matched on. An
/// unmatched key is reported as unmatched, never silently counted as zero.
fn match_branch<'a>(
    key: &str,
    branches: &Branches,
    stats: &'a [BranchStats],
) -> Option<&'a BranchStats> {
    let label = branches.source_map.iter().find_map(|(label, path)| {
        (path.file_stem().and_then(|s| s.to_str()) == Some(key)).then_some(label)
    })?;
    stats.iter().find(|s| &s.name == label)
}

#[must_use]
pub fn benchmarks(vocab: &Vocabulary) -> Vec<BenchmarkEntry> {
    let Some(registry) = vocab.subject("benchmark-registry") else {
        return Vec::new();
    };
    registry
        .children
        .iter()
        .map(|entry| BenchmarkEntry {
            key: entry.keys.clone(),
            anchor: entry.data.get("anchor").cloned(),
            last_benchmarked: entry.data.get("last-benchmarked").cloned(),
            cadence: entry.data.get("cadence").cloned(),
        })
        .collect()
}

#[must_use]
pub fn value_usage(vocab: &Vocabulary, topics: &[TopicMeta]) -> Vec<ValueUsage> {
    let mut out = Vec::new();
    for attribute in ["maturity", "volatility", "tool", "dimension"] {
        let Some(legal) = vocab.legal_values(attribute) else {
            continue;
        };
        let mut used: BTreeMap<String, usize> = BTreeMap::new();
        for meta in topics {
            let values: Vec<&String> = match attribute {
                "maturity" => meta.maturity.iter().collect(),
                "volatility" => meta.volatility.iter().collect(),
                "tool" => meta.tools.iter().collect(),
                _ => meta.dimensions.iter().collect(),
            };
            for v in values {
                if legal.contains(v) {
                    *used.entry(v.clone()).or_default() += 1;
                }
            }
        }
        let unused = legal
            .iter()
            .filter(|v| !used.contains_key(*v))
            .cloned()
            .collect();
        out.push(ValueUsage {
            attribute: attribute.to_string(),
            used,
            unused,
        });
    }
    out
}
