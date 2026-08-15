use std::{fs, path::Path};

use anyhow::Context;
use dita_ast::{TopicMeta, TopicType};
use dita_diagnostics::{Diagnostic, DiagnosticBag};

const XML_NS: &str = "http://www.w3.org/XML/1998/namespace";

/// Read what a `.dita` file declares about itself.
///
/// Reports facts and leaves judgement to the rules layer: a missing attribute
/// is `None`, not a default, and an unrecognised root element is
/// [`TopicType::Unknown`] rather than an error.
///
/// # Errors
///
/// Returns `Err` when the file cannot be used at all: the path does not
/// resolve, the file cannot be read, or it is not well-formed XML.
pub fn parse_topic(path: &Path) -> anyhow::Result<(TopicMeta, DiagnosticBag)> {
    let mut diag = DiagnosticBag::default();
    let canonical = path
        .canonicalize()
        .with_context(|| format!("cannot resolve path: {}", path.display()))?;
    let xml = fs::read_to_string(&canonical)
        .with_context(|| format!("cannot read file: {}", canonical.display()))?;
    // DITA topics carry a <!DOCTYPE ...>
    let opts = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };
    let doc = roxmltree::Document::parse_with_options(&xml, opts)
        .with_context(|| format!("XML parse error in: {}", canonical.display()))?;

    let root = doc.root_element();
    let topic_type = TopicType::from_root_element(root.tag_name().name());
    if topic_type == TopicType::Unknown {
        diag.push(Diagnostic::warning(
            &canonical,
            format!(
                "unrecognised root element <{}> — not treated as a topic type",
                root.tag_name().name()
            ),
        ));
    }

    // glossentry names its title <glossterm>; every other type uses <title>
    let title = root
        .children()
        .find(|n| n.has_tag_name("title") || n.has_tag_name("glossterm"))
        .and_then(|n| text_of(n))
        .unwrap_or_default();
    if title.is_empty() {
        diag.push(Diagnostic::warning(&canonical, "topic has no title"));
    }

    let mut meta = TopicMeta {
        path: canonical.clone(),
        id: root.attribute("id").map(str::to_string),
        title,
        topic_type,
        lang: root.attribute((XML_NS, "lang")).map(str::to_string),
        maturity: root.attribute("maturity").map(str::to_string),
        volatility: root.attribute("volatility").map(str::to_string),
        dimensions: split_values(root.attribute("dimension")),
        domain: None,
        planned_dimensions: Vec::new(),
        reviewed: None,
    };

    // governance metadata rides in prolog <data>; descendants are searched
    // rather than direct children because prolog nests it under <metadata>
    // in some topics
    for data in root.descendants().filter(|n| n.has_tag_name("data")) {
        let (Some(name), Some(value)) = (data.attribute("name"), data.attribute("value")) else {
            continue;
        };
        match name {
            "domain" => meta.domain = Some(value.to_string()),
            "planned-dimension" => meta.planned_dimensions.push(value.to_string()),
            "reviewed" => meta.reviewed = Some(value.to_string()),
            _ => {}
        }
    }

    Ok((meta, diag))
}

/// Concatenate a element's text, including text inside child elements — a
/// title may contain markup such as `<term>` or `<xmlelement>`.
fn text_of(node: roxmltree::Node) -> Option<String> {
    let text: String = node
        .descendants()
        .filter(roxmltree::Node::is_text)
        .filter_map(|n| n.text())
        .collect();
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn split_values(attr: Option<&str>) -> Vec<String> {
    attr.map(|v| v.split_whitespace().map(str::to_string).collect())
        .unwrap_or_default()
}
