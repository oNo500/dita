use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use dita_ast::{DitaMap, MapNode, MapRef, ProcessingRole, TopicHead, TopicRef};
use dita_diagnostics::{Diagnostic, DiagnosticBag};

/// Parse a `.ditamap` file, recursively expanding all `<mapref>` elements.
/// Returns the resolved map tree and any diagnostics (errors/warnings) collected.
///
/// Corresponds to DITA-OT's `MaprefModule.java` (197 lines) + `mapref.xsl`.
pub fn parse_map(path: &Path) -> anyhow::Result<(DitaMap, DiagnosticBag)> {
    let mut diag = DiagnosticBag::default();
    let mut visited = HashSet::new();
    let map = parse_map_file(path, &mut visited, &mut diag)?;
    Ok((map, diag))
}

fn parse_map_file(
    path: &Path,
    visited: &mut HashSet<PathBuf>,
    diag: &mut DiagnosticBag,
) -> anyhow::Result<DitaMap> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("cannot resolve path: {}", path.display()))?;

    // Circular mapref detection — mirrors DITA-OT's loop prevention logic
    if !visited.insert(canonical.clone()) {
        diag.push(Diagnostic::error(&canonical, "circular mapref detected"));
        return Ok(DitaMap {
            title: String::new(),
            path: canonical,
            lang: None,
            children: vec![],
        });
    }

    let base = canonical.parent().unwrap_or(Path::new(".")).to_path_buf();
    let xml = fs::read_to_string(&canonical)
        .with_context(|| format!("cannot read file: {}", canonical.display()))?;
    // DITA files always have `<!DOCTYPE ...>` declarations, so we must allow DTD.
    let opts = roxmltree::ParsingOptions { allow_dtd: true, ..roxmltree::ParsingOptions::default() };
    let doc = roxmltree::Document::parse_with_options(&xml, opts)
        .with_context(|| format!("XML parse error in: {}", canonical.display()))?;

    let root = doc.root_element();
    let title = root
        .children()
        .find(|n| n.has_tag_name("title"))
        .and_then(|n| n.text())
        .unwrap_or("")
        .to_string();
    // roxmltree stores xml:lang under the XML namespace URI
    let lang = root
        .attribute(("http://www.w3.org/XML/1998/namespace", "lang"))
        .map(str::to_string);
    let children = collect_children(root, &base, visited, diag);

    Ok(DitaMap { title, path: canonical, lang, children })
}

fn collect_children(
    node: roxmltree::Node,
    base: &Path,
    visited: &mut HashSet<PathBuf>,
    diag: &mut DiagnosticBag,
) -> Vec<MapNode> {
    let mut result = Vec::new();

    for child in node.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "mapref" => {
                let Some(href_str) = child.attribute("href") else {
                    diag.push(Diagnostic::warning(base, "mapref missing href attribute"));
                    continue;
                };
                let href = base.join(href_str);
                let role = match child.attribute("processing-role") {
                    Some("resource-only") => ProcessingRole::ResourceOnly,
                    _ => ProcessingRole::Normal,
                };
                // resource-only maprefs (e.g. subjectScheme) are recorded but not expanded
                if role == ProcessingRole::ResourceOnly {
                    result.push(MapNode::MapRef(MapRef { href, processing_role: role }));
                    continue;
                }
                // Normal maprefs: recursively expand inline
                match parse_map_file(&href, visited, diag) {
                    Ok(sub_map) => result.extend(sub_map.children),
                    Err(e) => diag.push(Diagnostic::error(&href, e.to_string())),
                }
            }
            "topicref" => {
                if let Some(href_str) = child.attribute("href") {
                    let href = base.join(href_str);
                    let nav_title = extract_nav_title(&child);
                    if !href.exists() {
                        diag.push(Diagnostic::error(
                            &href,
                            format!("referenced file not found: {}", href.display()),
                        ));
                    }
                    result.push(MapNode::TopicRef(TopicRef { href, nav_title }));
                }
            }
            "topichead" => {
                let nav_title = extract_nav_title(&child)
                    .unwrap_or_else(|| "(unnamed)".to_string());
                let children = collect_children(child, base, visited, diag);
                result.push(MapNode::TopicHead(TopicHead { nav_title, children }));
            }
            "title" => {} // already captured at map level
            _ => {}       // unknown/unsupported elements are silently skipped
        }
    }

    result
}

fn extract_nav_title(node: &roxmltree::Node) -> Option<String> {
    node.children()
        .find(|n| n.has_tag_name("topicmeta"))?
        .children()
        .find(|n| n.has_tag_name("navtitle"))?
        .text()
        .map(str::to_string)
}
