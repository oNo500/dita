use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct TopicMeta {
    pub path: PathBuf,
    pub id: String,
    pub title: String,
    pub topic_type: TopicType,
    pub maturity: Option<String>,
    pub volatility: Option<String>,
    pub dimension: Vec<String>,
    pub domain: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopicType {
    Concept,
    Reference,
    Task,
    Troubleshooting,
    GlossEntry,
    Unknown,
}
