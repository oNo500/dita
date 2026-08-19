//! 重复 topicref：同一篇 topic 在同一处编排里被引用两次。
//!
//! # 报什么，不报什么
//!
//! 同一个 topic 被多个 map 引用是**合法的**——一份内容多处编排正是 DITA 的用途，
//! 本库的交付物 map 与领域 map 引同一篇就是设计，不是事故。所以本模块只报两种
//! 形状，两种都在**同一处编排之内**：
//!
//! 一、**同 map 文件内重复**（[`DuplicateKind::SameMap`]）。复制粘贴 map 条目时
//!    的典型事故，没有任何合法读法：一个 map 说不出"这一篇在我这里有两个位置"。
//!
//! 二、**同一棵解析树内经不同 map 两次到达**（[`DuplicateKind::SameTree`]）。
//!    root map 底下 ai 与 web 两个领域 map 各引一次同一篇，展开后导航里就是两个
//!    节点。这一类判为要报，理由是后果与第一种完全一样：导航双份、`branch_stats`
//!    的分支计数重复、覆盖度按分支上卷时同一篇算两遍。而"一篇同时属于两个领域"
//!    这件事本身也是要人裁的——要么归属选错了，要么该改成一处正文加一处 xref。
//!
//! 不报的三种，逐条记明理由：
//!
//! - **跨编排单位重复**：`maps/deliverables/agent-rules.ditamap` 与 root 树各引
//!   一篇。两棵树各自都只到达它一次，正是"多处编排"，报了就是误报。
//! - **`processing-role="resource-only"` 的 topicref**：它不是导航节点，存在的
//!   目的就是把目标送进键空间；同一个 href 挂两个不同的 key 是合法的别名手法。
//!   本层只管导航树的重复，把这类算进来会误伤该手法。
//! - **只有 `@keyref` 没有 `@href` 的 topicref**：解析它要键空间，那是本层之上的
//!   一层。这是明知的盲区，不是通过——记在这里，将来键空间落地时补。
//!
//! 判定的粒度是 map **文件**，不是 map 节点：同一个领域 map 既被 root 展开一次、
//! 又被 `--maps-dir` 扫描单独解析一次，两次看到的内容一样，只登记一次。

use dita_ast::{DitaMap, MapNode, ProcessingRole};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

/// 重复的形状。两种都是缺陷信号，区别在于该去哪个文件改。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateKind {
    /// 同一个 map 文件内引了两次。
    SameMap,
    /// 同一棵解析树内经不同 map 两次到达。
    SameTree,
}

/// 一处重复引用。
#[derive(Debug)]
pub struct DuplicateRef {
    pub kind: DuplicateKind,
    /// 出问题的编排单位：`SameMap` 是那个 map 文件，`SameTree` 是树根 map。
    pub scope: PathBuf,
    /// 被重复引用的 topic。
    pub topic: PathBuf,
    /// 引用它的 map 文件，按树内出现顺序、去重后。`SameMap` 下只有一个元素
    /// （就是 `scope` 自己），`SameTree` 下至少两个。
    pub via: Vec<PathBuf>,
    /// 一共到达几次（`SameTree` 下可大于 `via` 的长度：某个 map 内部又重复了，
    /// 那一份由 `SameMap` 单独报）。
    pub count: usize,
}

/// 扫出全部重复 topicref。
///
/// `consulted` 是解析过的全部 map（root 树 ＋ `--maps-dir` 扫到的每一个）。
/// 树根由引用关系推出：没有被任何 map 以 mapref 引用的，就是一棵树的根——
/// 本库里是 `maps/root.ditamap` 与各交付物 map。只在树根上做第二种判定，
/// 领域 map 既作为 root 的子树、又作为独立 map 出现时才不会报两遍。
#[must_use]
pub fn duplicate_topicrefs(consulted: &[DitaMap]) -> Vec<DuplicateRef> {
    let mut trees: Vec<(PathBuf, Vec<Occurrence>)> = Vec::new();
    let mut referenced: BTreeSet<PathBuf> = BTreeSet::new();
    for map in consulted {
        let mut occurrences = Vec::new();
        let owner = canonical(&map.path);
        collect(&map.children, &owner, &mut occurrences, &mut referenced);
        trees.push((owner, occurrences));
    }

    let mut out = Vec::new();
    out.extend(same_map(&trees));
    out.extend(same_tree(&trees, &referenced));
    out
}

/// 一次到达：谁引的（map 文件），引到哪（topic）。
struct Occurrence {
    owner: PathBuf,
    topic: PathBuf,
}

/// 同一个 map 文件内的重复。
///
/// 同一个 owner 文件可能在多棵树里各出现一次（领域 map 既是 root 的子树，
/// 又被 `--maps-dir` 单独解析）。两次看到的是同一份内容，所以按首次出现的
/// 那棵树计数，之后再遇到就跳过——否则计数会翻倍，凭空造出重复。
fn same_map(trees: &[(PathBuf, Vec<Occurrence>)]) -> Vec<DuplicateRef> {
    let mut counted: BTreeMap<&Path, BTreeMap<&Path, usize>> = BTreeMap::new();
    for (_, occurrences) in trees {
        let mut this_tree: BTreeMap<&Path, BTreeMap<&Path, usize>> = BTreeMap::new();
        for occ in occurrences {
            *this_tree
                .entry(occ.owner.as_path())
                .or_default()
                .entry(occ.topic.as_path())
                .or_default() += 1;
        }
        for (owner, counts) in this_tree {
            counted.entry(owner).or_insert(counts);
        }
    }

    let mut out = Vec::new();
    for (owner, counts) in counted {
        for (topic, count) in counts {
            if count > 1 {
                out.push(DuplicateRef {
                    kind: DuplicateKind::SameMap,
                    scope: owner.to_path_buf(),
                    topic: topic.to_path_buf(),
                    via: vec![owner.to_path_buf()],
                    count,
                });
            }
        }
    }
    out
}

/// 同一棵树内经不同 map 两次到达。只在树根上算；单一 map 内部的重复由
/// [`same_map`] 报，这里不重复报。
fn same_tree(
    trees: &[(PathBuf, Vec<Occurrence>)],
    referenced: &BTreeSet<PathBuf>,
) -> Vec<DuplicateRef> {
    let mut out = Vec::new();
    for (root, occurrences) in trees {
        if referenced.contains(root) {
            continue; // 这棵"树"是别人的子树，它的重复在那棵树上报
        }
        let mut by_topic: BTreeMap<&Path, Vec<&Path>> = BTreeMap::new();
        for occ in occurrences {
            by_topic
                .entry(occ.topic.as_path())
                .or_default()
                .push(occ.owner.as_path());
        }
        for (topic, owners) in by_topic {
            let distinct: BTreeSet<&Path> = owners.iter().copied().collect();
            let owners_len = owners.len();
            if owners_len > 1 && distinct.len() > 1 {
                let mut via: Vec<PathBuf> = Vec::new();
                for owner in owners {
                    let owner = owner.to_path_buf();
                    if !via.contains(&owner) {
                        via.push(owner);
                    }
                }
                out.push(DuplicateRef {
                    kind: DuplicateKind::SameTree,
                    scope: root.clone(),
                    topic: topic.to_path_buf(),
                    count: owners_len,
                    via,
                });
            }
        }
    }
    out
}

/// 沿一棵已展开的树收集到达记录。`owner` 随 mapref 边界切换——被引 map 里的
/// topicref 属于那个文件，不属于引它的容器。
fn collect(
    nodes: &[MapNode],
    owner: &Path,
    out: &mut Vec<Occurrence>,
    referenced: &mut BTreeSet<PathBuf>,
) {
    for node in nodes {
        match node {
            MapNode::TopicRef(t) => {
                // resource-only 不是导航节点（且按 DITA 规则向下级联）
                if t.processing_role == ProcessingRole::ResourceOnly {
                    continue;
                }
                if let Some(href) = &t.href {
                    out.push(Occurrence {
                        owner: owner.to_path_buf(),
                        topic: canonical(href),
                    });
                }
                collect(&t.children, owner, out, referenced);
            }
            MapNode::TopicHead(h) => collect(&h.children, owner, out, referenced),
            MapNode::MapRef(m) => {
                let sub = canonical(&m.href);
                referenced.insert(sub.clone());
                collect(&m.children, &sub, out, referenced);
            }
        }
    }
}

fn canonical(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}
