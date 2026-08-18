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
            href: Some(PathBuf::from("topics/foo.dita")),
            keyref: None,
            nav_title: None,
            processing_role: ProcessingRole::default(),
            chunk: None,
            children: vec![],
        });
        assert!(matches!(node, MapNode::TopicRef(_)));
    }

    #[test]
    fn processing_role_defaults_to_normal() {
        assert_eq!(ProcessingRole::default(), ProcessingRole::Normal);
    }
}
