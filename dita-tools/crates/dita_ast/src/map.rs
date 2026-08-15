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

#[derive(Debug, Clone, PartialEq)]
pub struct TopicRef {
    pub href: PathBuf,
    pub nav_title: Option<String>,
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
