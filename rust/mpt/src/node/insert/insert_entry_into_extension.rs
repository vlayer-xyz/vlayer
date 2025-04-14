use super::{entry::Entry, utils::extract_common_prefix};
use crate::node::{
    Node, NodeError,
    insert::insert_entry_into_extension::from_extension_and_entry_empty_common_prefix::from_extension_and_entry_empty_common_prefix,
};

mod from_extension_and_entry_empty_common_prefix;

impl<D> Node<D> {
    pub(crate) fn insert_entry_into_extension(
        self,
        entry: impl Into<Entry>,
    ) -> Result<Node<D>, NodeError> {
        let Node::Extension(key, child_node) = self.clone() else {
            unreachable!("insert_entry_into_extension is used only for Extension nodes");
        };

        let entry = entry.into();

        let (common_prefix, remaining_extension_key, remaining_entry_key) =
            extract_common_prefix(&key, &entry.key);

        // ![Remaining extension key empty](../../../images/into_extension_0.png)
        if remaining_extension_key.is_empty() {
            let child_node = child_node.insert(remaining_entry_key, entry.value)?;
            return Ok(Node::extension(common_prefix, child_node));
        }

        // ![Common prefix empty](../../../images/into_extension_1.png)
        if common_prefix.is_empty() {
            return from_extension_and_entry_empty_common_prefix(self, entry);
        }

        let child_node = from_extension_and_entry_empty_common_prefix(
            Node::extension(remaining_extension_key, *child_node),
            (remaining_entry_key, entry.value),
        )?;

        // ![Common prefix nonempty](../../../images/into_extension_2.png)
        Ok(Node::extension(common_prefix, child_node))
    }
}

#[cfg(test)]
mod tests {
    use crate::KeccakNode as Node;

    #[test]
    #[should_panic(expected = "insert_entry_into_extension is used only for Extension nodes")]
    fn unreachable() {
        let null = Node::null();
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
            Node::leaf([], [43]),
        );

        assert_eq!(updated_node, expected_node);
        Ok(())
    }

    mod common_prefix_non_empty {
        use super::*;

        #[test]
        fn into_child_node() -> anyhow::Result<()> {
            let child_node = Node::branch_with_value([42]);
            let node = Node::extension([0x0], child_node.clone());

            let updated_node = node.insert_entry_into_extension(([0x0, 0x1], [43]))?;
            let updated_child_node = child_node.insert([0x1], [43])?;

            assert_eq!(updated_node, Node::extension([0x0], updated_child_node));
            Ok(())
        }

        #[test]
        fn into_extension_node_directly() -> anyhow::Result<()> {
            let extension = Node::extension([0x0, 0x0], Node::branch_with_value([42]));
            let node = extension.insert_entry_into_extension(([0x0, 0x1], [43]))?;

            let child_node = Node::branch_with_two_children(
                0,
                Node::branch_with_value([42]),
                1,
                Node::leaf([], [43]),
            );
            let expected_node = Node::extension([0x0], child_node);

            assert_eq!(node, expected_node);
            Ok(())
        }
    }
}
