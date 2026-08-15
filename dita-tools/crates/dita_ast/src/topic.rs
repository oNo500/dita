use std::path::PathBuf;

/// What a topic file declares about itself.
///
/// Facts only: a missing `@maturity` is `None` here, not `draft`. The default
/// lives in the subject scheme (`<defaultSubject>`) and applying it is the
/// rules layer's job — a parser that silently substitutes defaults makes
/// "author forgot to tag this" indistinguishable from "author chose the
/// default", and R2 exists precisely to catch the former.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicMeta {
    pub path: PathBuf,
    pub id: Option<String>,
    pub title: String,
    pub topic_type: TopicType,
    pub lang: Option<String>,
    pub maturity: Option<String>,
    pub volatility: Option<String>,
    /// `@dimension`, whitespace-separated (specialized from `@props`).
    pub dimensions: Vec<String>,
    /// `<data name="domain">` — the hand-written domain claim. Structural
    /// domain membership comes from the map tree instead; this is kept for
    /// cross-checking.
    pub domain: Option<String>,
    /// `<data name="planned-dimension">` — a landscape topic's declaration of
    /// the dimensions its domain plans to cover. Coverage is measured against
    /// this.
    pub planned_dimensions: Vec<String>,
    /// `<data name="reviewed">` — the date sources were last checked.
    pub reviewed: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopicType {
    Concept,
    Reference,
    Task,
    Troubleshooting,
    GlossEntry,
    /// A generic `<topic>`. Legal DITA; whether this library allows it is a
    /// rules-layer question, not a parsing one.
    Topic,
    /// Root element this parser does not recognise as a topic type.
    Unknown,
}

impl TopicType {
    #[must_use]
    pub fn from_root_element(name: &str) -> Self {
        match name {
            "concept" => Self::Concept,
            "reference" => Self::Reference,
            "task" => Self::Task,
            "troubleshooting" => Self::Troubleshooting,
            "glossentry" => Self::GlossEntry,
            "topic" => Self::Topic,
            _ => Self::Unknown,
        }
    }

    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Concept => "concept",
            Self::Reference => "reference",
            Self::Task => "task",
            Self::Troubleshooting => "troubleshooting",
            Self::GlossEntry => "glossentry",
            Self::Topic => "topic",
            Self::Unknown => "unknown",
        }
    }
}
