use crate::domain::Branches;
use dita_ast::TopicMeta;
use std::collections::{BTreeMap, BTreeSet};

/// What a branch holds, for deciding what to write next.
#[derive(Debug)]
pub struct BranchStats {
    pub name: String,
    pub topics: usize,
    /// Whether any topic here declares planned dimensions — i.e. whether the
    /// branch has a landscape (R9). Reported as an observation rather than a
    /// warning: purely organisational branches such as the glossary are not
    /// domains and are not supposed to have one.
    pub has_landscape: bool,
    pub by_type: BTreeMap<String, usize>,
    pub by_maturity: BTreeMap<String, usize>,
    pub by_volatility: BTreeMap<String, usize>,
}

/// Dimension coverage of one technology domain.
///
/// Semantics deliberately identical to `kb/scripts/dimension-coverage.py`, so
/// that the two implementations can be checked against each other: coverage
/// counts only planned dimensions, and anything covered outside the plan is
/// listed separately rather than counted.
#[derive(Debug)]
pub struct DomainCoverage {
    pub domain: String,
    pub planned: BTreeSet<String>,
    pub covered: BTreeSet<String>,
    pub blind: BTreeSet<String>,
    pub outside_plan: BTreeSet<String>,
    /// Branches the domain's topics hang under. More than one is a smell: a
    /// technology domain should live in one branch.
    pub branches: BTreeSet<String>,
    pub topics: usize,
}

impl DomainCoverage {
    #[must_use]
    pub fn percent(&self) -> usize {
        if self.planned.is_empty() {
            0
        } else {
            self.covered.len() * 100 / self.planned.len()
        }
    }
}

/// `None` maturity is reported as unmarked rather than as the scheme's default:
/// "not tagged" is what R2 looks for, so it must stay visible here.
const UNMARKED: &str = "（未标）";

#[must_use]
pub fn branch_stats(branches: &Branches, topics: &[TopicMeta]) -> Vec<BranchStats> {
    let by_path: BTreeMap<_, _> = topics.iter().map(|t| (t.path.clone(), t)).collect();
    branches
        .topics
        .iter()
        .map(|(name, paths)| {
            let mut stats = BranchStats {
                name: name.clone(),
                topics: paths.len(),
                has_landscape: false,
                by_type: BTreeMap::new(),
                by_maturity: BTreeMap::new(),
                by_volatility: BTreeMap::new(),
            };
            for path in paths {
                let Some(meta) = by_path.get(path) else {
                    continue;
                };
                stats.has_landscape |= !meta.planned_dimensions.is_empty();
                bump(&mut stats.by_type, meta.topic_type.as_str());
                bump(
                    &mut stats.by_maturity,
                    meta.maturity.as_deref().unwrap_or(UNMARKED),
                );
                bump(
                    &mut stats.by_volatility,
                    meta.volatility.as_deref().unwrap_or(UNMARKED),
                );
            }
            stats
        })
        .collect()
}

#[must_use]
pub fn domain_coverage(branches: &Branches, topics: &[TopicMeta]) -> Vec<DomainCoverage> {
    let mut planned: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut covered: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut in_branches: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    for meta in topics {
        // a topic that declares no domain takes no part, exactly as in the
        // Python implementation
        let Some(domain) = &meta.domain else { continue };
        *counts.entry(domain.clone()).or_default() += 1;
        // a planned entry exists only when a landscape declared dimensions —
        // matching the Python implementation, which reports coverage only for
        // domains that have a plan. Creating an empty entry for every declared
        // domain made nodes without a landscape read "全景 0/0", which claims a
        // landscape exists when none does.
        if !meta.planned_dimensions.is_empty() {
            planned
                .entry(domain.clone())
                .or_default()
                .extend(meta.planned_dimensions.iter().cloned());
        }
        covered
            .entry(domain.clone())
            .or_default()
            .extend(meta.dimensions.iter().cloned());
        in_branches
            .entry(domain.clone())
            .or_default()
            .extend(branches.of(&meta.path).iter().cloned());
    }

    planned
        .into_iter()
        .map(|(domain, planned)| {
            let all_covered = covered.remove(&domain).unwrap_or_default();
            let covered_in_plan: BTreeSet<_> = all_covered.intersection(&planned).cloned().collect();
            DomainCoverage {
                blind: planned.difference(&covered_in_plan).cloned().collect(),
                outside_plan: all_covered.difference(&planned).cloned().collect(),
                branches: in_branches.remove(&domain).unwrap_or_default(),
                topics: counts.get(&domain).copied().unwrap_or(0),
                covered: covered_in_plan,
                planned,
                domain,
            }
        })
        .collect()
}

fn bump(map: &mut BTreeMap<String, usize>, key: &str) {
    *map.entry(key.to_string()).or_default() += 1;
}
