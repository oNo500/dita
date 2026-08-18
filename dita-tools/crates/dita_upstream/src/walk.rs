use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::Context;
use dita_ast::{MapNode, ProcessingRole};

use crate::{Entry, Index, KeySpace, Source};

pub fn build(sources: &[Source]) -> anyhow::Result<Index> {
    let mut ctx = Ctx {
        entries: Vec::new(),
        notes: Vec::new(),
        seen: HashSet::new(),
        keys: KeySpace::default(),
        root: PathBuf::new(),
        flavor_index: 0,
    };

    for (i, source) in sources.iter().enumerate() {
        let root = source
            .root
            .canonicalize()
            .with_context(|| format!("来源根目录不存在：{}", source.root.display()))?;
        let (map, diag) = dita_parser::parse_map(&source.entry)?;
        ctx.root = root;
        ctx.flavor_index = i;
        ctx.keys = KeySpace::build(&source.entry);

        let before = ctx.entries.len();
        walk(&map.children, "", false, sources, &mut ctx);
        ctx.notes.push(format!(
            "{}：{} 个节点（键空间 {} 个键 / {} 张 map，解析诊断 {} 条）",
            source.entry.display(),
            ctx.entries.len() - before,
            ctx.keys.len(),
            ctx.keys.maps_scanned,
            diag.items.len(),
        ));
    }

    ctx.entries
        .sort_by(|a, b| (&a.source, &a.path, &a.title).cmp(&(&b.source, &b.path, &b.title)));
    Ok(Index {
        entries: ctx.entries,
        notes: ctx.notes,
    })
}

struct Ctx {
    entries: Vec<Entry>,
    notes: Vec<String>,
    /// 已收录的文件（跨来源）。同一篇被多张 map 引用是常态，节点只算一次
    seen: HashSet<PathBuf>,
    keys: KeySpace,
    root: PathBuf,
    flavor_index: usize,
}

/// `chunked`：祖先里有 `chunk="to-content"`——整棵子树被合成一个页面，
/// 这些节点在站点上没有自己的地址（DITA-OT 的术语表就是这么发布的）。
fn walk(nodes: &[MapNode], parent: &str, chunked: bool, sources: &[Source], ctx: &mut Ctx) {
    for node in nodes {
        match node {
            // mapref 是透明的：DITA 规定被引 map 的 <title> 不产生导航节点，
            // 它的层级并入容器。把它当节点会凭空造出"Specialization"这类
            // 上游并不存在的条目——而索引的用途正是核实节点是否存在
            MapNode::MapRef(m) => walk(&m.children, parent, chunked, sources, ctx),
            MapNode::TopicHead(h) => {
                emit(&h.nav_title, parent, None, chunked, sources, ctx);
                walk(&h.children, &h.nav_title.clone(), chunked, sources, ctx);
            }
            MapNode::TopicRef(t) => {
                // resource-only 及其整棵子树不是导航节点：它们只为进键空间或
                // 被 conref 而存在，站点上没有对应页面（processing-role 按
                // DITA 规则向下级联，所以整支跳过）
                if t.processing_role == ProcessingRole::ResourceOnly {
                    continue;
                }
                let target = t.href.clone().or_else(|| {
                    t.keyref
                        .as_deref()
                        .and_then(|k| ctx.keys.resolve(k).map(Path::to_path_buf))
                });
                let Some(target) = target else {
                    if let Some(k) = &t.keyref {
                        ctx.notes.push(format!("键解析不到，节点跳过：{k}"));
                    }
                    // 纯分组 topicref 自己不是节点，孩子挂到当前父节点下
                    walk(&t.children, parent, chunked, sources, ctx);
                    continue;
                };
                if target.extension().and_then(|e| e.to_str()) == Some("ditamap") {
                    // keyref 指到一张 map：与 mapref 同样透明
                    walk(&t.children, parent, chunked, sources, ctx);
                    continue;
                }
                let title = title_of(&target, t.nav_title.as_deref(), ctx);
                emit(&title, parent, Some(&target), chunked, sources, ctx);
                // 带 chunk="to-content" 的节点自己仍有页面，被并进去的是它的后代
                let chunked = chunked || t.chunk.as_deref() == Some("to-content");
                walk(&t.children, &title, chunked, sources, ctx);
            }
        }
    }
}

/// 节点标题：以 topic 自己的 `<title>` 为准，navtitle 只是兜底。
///
/// navtitle 是"在这张 map 里叫什么"，同一篇在不同 map 下可以不同；索引记的是
/// 节点的标准叫法，那只能来自 topic 本身。
fn title_of(path: &Path, nav_title: Option<&str>, ctx: &mut Ctx) -> String {
    match dita_parser::parse_topic(path) {
        Ok((meta, _)) if !meta.title.is_empty() => normalize(&meta.title),
        Ok(_) => {
            ctx.notes
                .push(format!("topic 无标题，退回 navtitle：{}", path.display()));
            nav_title.map(normalize).unwrap_or_default()
        }
        Err(e) => {
            ctx.notes.push(format!("topic 读不了：{e}"));
            nav_title.map(normalize).unwrap_or_default()
        }
    }
}

/// 标题里的换行与连续空白压成单空格。
///
/// 源文件为了排版会把标题折行（`<mainbooktitle>Darwin … Version\n  2.0`），
/// 原样写进 tsv 会把一行撑成两行，制表符更会撑出一列。
fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn emit(
    title: &str,
    parent: &str,
    path: Option<&Path>,
    chunked: bool,
    sources: &[Source],
    ctx: &mut Ctx,
) {
    if title.is_empty() {
        return;
    }
    let flavor = &sources[ctx.flavor_index].flavor;
    let (rel, canonical) = match path {
        Some(p) => {
            let Ok(canonical) = p.canonicalize() else {
                ctx.notes
                    .push(format!("路径解析不了，节点跳过：{}", p.display()));
                return;
            };
            let Ok(rel) = canonical.strip_prefix(&ctx.root) else {
                ctx.notes.push(format!(
                    "文件在来源根目录之外，节点跳过：{}",
                    canonical.display()
                ));
                return;
            };
            (rel.to_path_buf(), Some(canonical))
        }
        // 导航节点（topichead）没有文件：有标题、有父子关系，但无 path 无 url
        None => (PathBuf::new(), None),
    };
    if let Some(canonical) = canonical {
        if !ctx.seen.insert(canonical) {
            return;
        }
    }
    ctx.entries.push(Entry {
        source: flavor.source_id(&rel),
        title: title.to_string(),
        parent: normalize(parent),
        path: rel.to_string_lossy().replace('\\', "/"),
        url: if chunked {
            String::new()
        } else {
            flavor.url(&rel)
        },
    });
}
