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

#[derive(Debug, Clone, PartialEq)]
pub struct MapRef {
    pub href: PathBuf,
    pub processing_role: ProcessingRole,
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
