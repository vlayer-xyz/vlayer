use crate::node::{Node, NodeError, insert::entry::Entry};

#[allow(unused)]
pub(crate) fn from_extension_and_entry_empty_common_prefix<D>(
    extension: Node<D>,
    entry: impl Into<Entry>,
) -> Result<Node<D>, NodeError> {
    let Node::Extension(key, child_node) = extension else {
        unreachable!("from_extension_and_entry_no_common_prefix is used only for Extension nodes");
    };
    let entry = entry.into();

    // `key` is not empty, since extension cannot have an empty key
    let branch = Node::branch_with_child_node(&key, *child_node);
    let updated_branch = branch.insert(&*entry.key, entry.value)?;

    Ok(updated_branch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KeccakNode as Node;

    mod empty_entry_key {
        use super::*;

        #[test]
        fn one_nibble_extension() -> anyhow::Result<()> {
            let extension = Node::extension([0x0], Node::branch_with_value([42]));

            let node = from_extension_and_entry_empty_common_prefix(extension, ([], [43]))?;

            let child = Node::branch_with_value([42]);
            let expected_node = Node::branch_with_child_and_value(0, child, [43]);

            assert_eq!(node, expected_node);
            Ok(())
        }

        #[test]
        fn multiple_nibbles_extension() -> anyhow::Result<()> {
            let extension = Node::extension([0x0, 0x0], Node::branch_with_value([42]));

            let node = from_extension_and_entry_empty_common_prefix(extension, ([], [43]))?;

            let child = Node::extension([0x0], Node::branch_with_value([42]));
            let expected_node = Node::branch_with_child_and_value(0, child, [43]);

            assert_eq!(node, expected_node);
            Ok(())
        }
    }

    mod non_empty_entry_key {
        use super::*;

        #[test]
        fn one_nibble_extension() -> anyhow::Result<()> {
            let extension = Node::extension([0x0], Node::branch_with_value([42]));

            let node = from_extension_and_entry_empty_common_prefix(extension, ([0x1], [43]))?;

            let expected_node = Node::branch_with_two_children(
                0,
                Node::branch_with_value([42]),
                1,
                Node::leaf([], [43]),
            );

            assert_eq!(node, expected_node);
            Ok(())
        }

        #[test]
        fn multiple_nibbles_extension() -> anyhow::Result<()> {
            let extension = Node::extension([0x0, 0x0], Node::branch_with_value([42]));

            let node = from_extension_and_entry_empty_common_prefix(extension, ([0x1], [43]))?;

            let expected_node = Node::branch_with_two_children(
                0,
                Node::extension([0x0], Node::branch_with_value([42])),
                1,
                Node::leaf([], [43]),
            );

            assert_eq!(node, expected_node);
            Ok(())
        }
    }
}
