use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use dita_ast::{DitaMap, MapNode, MapRef, ProcessingRole, TopicHead, TopicRef};
use dita_diagnostics::{Diagnostic, DiagnosticBag};

/// Parse a `.ditamap` file, recursively resolving all `<mapref>` elements.
/// Returns the resolved map tree and any diagnostics (errors/warnings) collected.
///
/// Corresponds to DITA-OT's `MaprefModule.java` (197 lines) + `mapref.xsl`,
/// with one deliberate difference: referenced maps stay their own nodes instead
/// of being spliced into the parent (see `MapRef`).
///
/// # Errors
///
/// Returns `Err` when the root map itself cannot be used: the path does not
/// resolve, the file cannot be read, or it is not well-formed XML. Problems in
/// referenced maps are diagnostics, not errors — one broken branch must not
/// take down the whole tree.
pub fn parse_map(path: &Path) -> anyhow::Result<(DitaMap, DiagnosticBag)> {
    let mut diag = DiagnosticBag::default();
    let mut ancestors = Vec::new();
    let map = parse_map_file(path, &mut ancestors, &mut diag)?;
    Ok((map, diag))
}

fn parse_map_file(
    path: &Path,
    ancestors: &mut Vec<PathBuf>,
    diag: &mut DiagnosticBag,
) -> anyhow::Result<DitaMap> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("cannot resolve path: {}", path.display()))?;

    // A cycle is a map that references one of its own ancestors. The same map
    // reached twice through different parents is a diamond, which DITA permits
    // — tracking a global visited set would misreport those as cycles.
    if ancestors.contains(&canonical) {
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
    let opts = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };
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
    // 推栈放在所有可失败步骤之后：早推而中途 `?` 返回，栈上会留下这条路径，
    // 后面再引用同一个 map 就被误判成环——菱形引用 + 子 map 解析失败正好触发，
    // 且真正的原因（XML 错误）会被"circular mapref"盖掉。
    ancestors.push(canonical.clone());
    let children = collect_children(root, &base, ancestors, diag);
    ancestors.pop();

    Ok(DitaMap {
        title,
        path: canonical,
        lang,
        children,
    })
}

/// Bookmap specializations of `<topicref>`. They differ from `<topicref>` in
/// where they may appear, not in what they mean structurally, so the tree view
/// treats them identically — without this the OASIS specification bookmap
/// parses to an empty tree, since every one of its branches hangs off a
/// `<chapter>`.
///
/// `<frontmatter>` / `<backmatter>` / `<notices>` are deliberately absent:
/// covers and notices are not nodes of the subject tree.
const TOPICREF_LIKE: [&str; 6] = [
    "topicref", "chapter", "appendix", "part", "preface", "glossref",
];

fn collect_children(
    node: roxmltree::Node,
    base: &Path,
    ancestors: &mut Vec<PathBuf>,
    diag: &mut DiagnosticBag,
) -> Vec<MapNode> {
    let mut result = Vec::new();

    for child in node.children().filter(roxmltree::Node::is_element) {
        let name = child.tag_name().name();
        // `<topicref href="x.ditamap" format="ditamap">` is a mapref spelled the
        // long way; DITA treats the two the same and so must this
        let is_mapref = name == "mapref"
            || (TOPICREF_LIKE.contains(&name) && child.attribute("format") == Some("ditamap"));
        match if is_mapref { "mapref" } else { name } {
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
                    result.push(MapNode::MapRef(MapRef {
                        href,
                        processing_role: role,
                        title: None,
                        children: vec![],
                    }));
                    continue;
                }
                // Normal maprefs: resolve, but keep the referenced map as its own
                // node so that an empty one stays visible in the tree
                match parse_map_file(&href, ancestors, diag) {
                    Ok(sub_map) => result.push(MapNode::MapRef(MapRef {
                        href,
                        processing_role: role,
                        title: Some(sub_map.title).filter(|t| !t.is_empty()),
                        children: sub_map.children,
                    })),
                    Err(e) => diag.push(Diagnostic::error(&href, e.to_string())),
                }
            }
            n if TOPICREF_LIKE.contains(&n) => {
                let processing_role = match child.attribute("processing-role") {
                    Some("resource-only") => ProcessingRole::ResourceOnly,
                    _ => ProcessingRole::Normal,
                };
                // an external target is a URL, not a path — joining it onto the
                // map's directory would fabricate a local file that then gets
                // reported missing
                let external = child.attribute("scope") == Some("external");
                let href = child
                    .attribute("href")
                    .filter(|_| !external)
                    .map(|h| base.join(h));
                if let Some(href) = &href {
                    if !href.exists() {
                        diag.push(Diagnostic::error(
                            href,
                            format!("referenced file not found: {}", href.display()),
                        ));
                    }
                }
                result.push(MapNode::TopicRef(TopicRef {
                    href,
                    keyref: child.attribute("keyref").map(str::to_string),
                    nav_title: extract_nav_title(&child),
                    processing_role,
                    chunk: child.attribute("chunk").map(str::to_string),
                    // a topicref nested in a topicref is DITA's way of nesting
                    // without a topichead — dropping these loses whole subtrees
                    children: collect_children(child, base, ancestors, diag),
                }));
            }
            "topichead" => {
                let nav_title =
                    extract_nav_title(&child).unwrap_or_else(|| "(unnamed)".to_string());
                let children = collect_children(child, base, ancestors, diag);
                result.push(MapNode::TopicHead(TopicHead {
                    nav_title,
                    children,
                }));
            }
            // title is already captured at map level; anything else is an
            // element this view has no use for
            _ => {}
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
