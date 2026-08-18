use crate::{DitaMap, MapNode, MapRef, TopicHead, TopicRef};

/// A visitor that traverses the DITA map AST.
///
/// Override only the methods you care about. The default implementations
/// call the corresponding `walk_*` function to keep recursing into children.
///
/// # Example
///
/// ```rust
/// use dita_ast::visit::{Visit, walk_dita_map};
/// use dita_ast::{DitaMap, TopicRef};
///
/// struct TopicCounter(usize);
///
/// impl Visit for TopicCounter {
///     fn visit_topic_ref(&mut self, node: &TopicRef) {
///         self.0 += 1;
///         dita_ast::visit::walk_topic_ref(self, node); // 别忘了递归子 topicref
///     }
/// }
///
/// // let mut counter = TopicCounter(0);
/// // walk_dita_map(&mut counter, &map);
/// // println!("{} topics", counter.0);
/// ```
pub trait Visit: Sized {
    fn visit_dita_map(&mut self, node: &DitaMap) {
        walk_dita_map(self, node);
    }
    fn visit_map_node(&mut self, node: &MapNode) {
        walk_map_node(self, node);
    }
    fn visit_topic_ref(&mut self, node: &TopicRef) {
        walk_topic_ref(self, node);
    }
    fn visit_map_ref(&mut self, node: &MapRef) {
        walk_map_ref(self, node);
    }
    fn visit_topic_head(&mut self, node: &TopicHead) {
        walk_topic_head(self, node);
    }
}

// ── walk functions ──────────────────────────────────────────────────────────
// Generic over V: Visit so they work from inside default trait methods
// without needing trait object dispatch.

pub fn walk_dita_map<V: Visit>(v: &mut V, map: &DitaMap) {
    for child in &map.children {
        v.visit_map_node(child);
    }
}

pub fn walk_map_node<V: Visit>(v: &mut V, node: &MapNode) {
    match node {
        MapNode::TopicRef(n) => v.visit_topic_ref(n),
        MapNode::MapRef(n) => v.visit_map_ref(n),
        MapNode::TopicHead(n) => v.visit_topic_head(n),
    }
}

pub fn walk_map_ref<V: Visit>(v: &mut V, map_ref: &MapRef) {
    for child in &map_ref.children {
        v.visit_map_node(child);
    }
}

pub fn walk_topic_ref<V: Visit>(v: &mut V, topic_ref: &TopicRef) {
    for child in &topic_ref.children {
        v.visit_map_node(child);
    }
}

pub fn walk_topic_head<V: Visit>(v: &mut V, head: &TopicHead) {
    for child in &head.children {
        v.visit_map_node(child);
    }
}
