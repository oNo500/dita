use dita_ast::{
    DitaMap, MapNode, TopicHead,
    visit::{Visit, walk_dita_map},
};
use dita_diagnostics::{Diagnostic, DiagnosticBag};

/// Report `topichead` wrappers whose navtitle has drifted from the title of the
/// map they wrap.
///
/// Referencing a map does not produce a navigation node for it — the referenced
/// hierarchy is merged into the container — so wrapping a `mapref` in a
/// `topichead` is how a branch keeps its name in published output. The price is
/// that the name then exists twice, and the copy is free to drift. Checking it
/// is what makes that price affordable.
pub fn check_group_titles(map: &DitaMap, diag: &mut DiagnosticBag) {
    struct Checker<'a>(&'a mut DiagnosticBag);

    impl Visit for Checker<'_> {
        fn visit_topic_head(&mut self, node: &TopicHead) {
            // only a wrapper — a topichead with other content is a real
            // grouping node, not a stand-in for a map's title
            if let [MapNode::MapRef(m)] = node.children.as_slice() {
                if let Some(title) = &m.title {
                    if title != &node.nav_title {
                        self.0.push(Diagnostic::warning(
                            &m.href,
                            format!(
                                "topichead navtitle \"{}\" differs from the referenced map title \"{title}\"",
                                node.nav_title
                            ),
                        ));
                    }
                }
            }
            dita_ast::visit::walk_topic_head(self, node);
        }
    }

    let mut checker = Checker(diag);
    walk_dita_map(&mut checker, map);
}
