use crate::node::insert::insert_entry_into_extension::from_extension_and_entry_empty_common_prefix::from_extension_and_entry_empty_common_prefix;
use crate::node::{Node, NodeError};

use super::entry::Entry;
use super::utils::extract_common_prefix;

mod from_extension_and_entry_empty_common_prefix;

impl Node {
    pub(crate) fn insert_entry_into_extension(
        self,
        entry: impl Into<Entry>,
    ) -> Result<Node, NodeError> {
        let Node::Extension(key, child_node) = self else {
            unreachable!("insert_entry_into_extension is used only for Extension nodes");
        };

        let entry = entry.into();
        let (common_prefix, remaining_extension_key, remaining_entry_key) =
            extract_common_prefix(&key, &entry.key);

        if remaining_extension_key.is_empty() {
            let child_node = child_node.insert(remaining_entry_key, entry.value)?;
            return Ok(Node::extension(common_prefix, child_node));
        }

        if common_prefix.is_empty() {
            let remaining_extension = Node::extension(remaining_extension_key, *child_node);
            return from_extension_and_entry_empty_common_prefix(remaining_extension, entry);
        }

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "insert_entry_into_extension is used only for Extension nodes")]
    fn unreachable() {
        let null = Node::Null;
        null.insert_entry_into_extension(([], [42])).unwrap();
    }

    #[test]
    fn common_prefix_empty() -> anyhow::Result<()> {
        let node = Node::extension([0x0], Node::branch_with_value([42]));
        let updated_node = node.insert_entry_into_extension(([0x1], [43]))?;

        let expected_node = Node::branch_with_two_children(
            0,
            Node::branch_with_value([42]),
            1,
            Node::branch_with_value([43]),
        );

        assert_eq!(updated_node, expected_node);
        Ok(())
    }

    mod common_prefix_non_empty {
        use super::*;

        #[test]
        // In case where extension nibbles is a prefix of inserted key
        // we delegate insertion to the child node.
        // We test this by comparing the result of inserting to the child node directly.
        fn insert_into_child_node() -> anyhow::Result<()> {
            let child_node = Node::branch_with_value([42]);
            let node = Node::extension([0x0], child_node.clone());

            let updated_node = node.insert_entry_into_extension(([0x0, 0x1], [43]))?;
            let updated_child_node = child_node.insert([0x1], [43])?;

            assert_eq!(updated_node, Node::extension([0x0], updated_child_node));
            Ok(())
        }
    }
}
