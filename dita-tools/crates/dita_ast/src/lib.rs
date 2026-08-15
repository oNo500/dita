pub mod map;
pub mod topic;
pub mod visit;


pub use map::{DitaMap, MapNode, MapRef, ProcessingRole, TopicHead, TopicRef};
pub use topic::{TopicMeta, TopicType};
pub use visit::Visit;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn map_node_holds_topic_ref() {
        let node = MapNode::TopicRef(TopicRef {
            href: PathBuf::from("topics/foo.dita"),
            nav_title: None,
        });
        assert!(matches!(node, MapNode::TopicRef(_)));
    }

    #[test]
    fn processing_role_defaults_to_normal() {
        assert_eq!(ProcessingRole::default(), ProcessingRole::Normal);
    }
}
