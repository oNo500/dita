use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct DitaMap {
    pub title: String,
    pub path: PathBuf,
    pub lang: Option<String>,
    pub children: Vec<MapNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MapNode {
    TopicRef(TopicRef),
    MapRef(MapRef),
    TopicHead(TopicHead),
}

/// A `<topicref>` — or one of its bookmap specializations (`<chapter>`,
/// `<appendix>`, `<part>`, `<preface>`).
///
/// Both `href` and `keyref` are optional because DITA allows either, or
/// neither: a topicref that only carries `@keyref` names its target through
/// the key space, and one with neither is a pure grouping node. Resolving a
/// keyref needs a key space, which is a layer above parsing — the parser
/// records the reference and leaves the lookup to whoever has the keys.
///
/// `children` exists because nesting a topicref inside a topicref is how DITA
/// expresses hierarchy without a `<topichead>`; upstream sources (the OASIS
/// specification, DITA-OT's docsrc) use it throughout.
#[derive(Debug, Clone, PartialEq)]
pub struct TopicRef {
    pub href: Option<PathBuf>,
    pub keyref: Option<String>,
    pub nav_title: Option<String>,
    /// `@processing-role`。`resource-only` 的 topicref 只是让目标进键空间／
    /// 被 conref，不是导航节点（且按 DITA 规则向下级联）。
    pub processing_role: ProcessingRole,
    /// `@chunk`。`to-content` 表示整棵子树合成一个页面——子节点因此没有
    /// 自己的地址，这是判断"节点有没有独立 URL"的唯一有据可依的信号。
    pub chunk: Option<String>,
    pub children: Vec<MapNode>,
}

/// A `<mapref>` and, when it was expanded, the referenced map's title and
/// resolved children.
///
/// The referenced map is kept as its own node rather than being spliced into
/// the parent's children: an IA view has to be able to say "this domain exists
/// and is empty", which is impossible once a childless map has been inlined
/// into nothing. Publishing pipelines flatten maprefs; this is not one.
///
/// `resource-only` maprefs (subject schemes, key definition maps) are recorded
/// but never expanded, so their `title` and `children` stay empty.
#[derive(Debug, Clone, PartialEq)]
pub struct MapRef {
    pub href: PathBuf,
    pub processing_role: ProcessingRole,
    pub title: Option<String>,
    pub children: Vec<MapNode>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProcessingRole {
    #[default]
    Normal,
    ResourceOnly,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopicHead {
    pub nav_title: String,
    pub children: Vec<MapNode>,
}
