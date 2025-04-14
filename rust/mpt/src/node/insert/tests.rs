#[cfg(test)]
mod insert {
    use crate::node::{KeccakNode as Node, NodeError, insert::entry::Entry};

    #[test]
    #[should_panic(expected = "Cannot insert into a digest node")]
    fn digest() {
        let node = Node::digest(Default::default());
        node.insert([0x0], [42]).unwrap();
    }

    mod into_null {
        use super::*;

        #[test]
        fn empty_key() -> anyhow::Result<()> {
            let node = Node::null();
            assert_eq!(node.insert([], [42])?, Node::leaf([], [42]));
            Ok(())
        }

        #[test]
        fn short_key() -> anyhow::Result<()> {
            let node = Node::null();
            assert_eq!(node.insert([0x0], [42])?, Node::leaf([0x0], [42]));
            Ok(())
        }

        #[test]
        fn long_key() -> anyhow::Result<()> {
            let node = Node::null();
            assert_eq!(node.insert([0x0, 0x0], [42])?, Node::leaf([0x0, 0x0], [42]));
            Ok(())
        }

        #[test]
        fn double_insert() -> anyhow::Result<()> {
            let node = Node::null();
            let updated_node = node.insert([0x1, 0x0], [42])?.insert([0x2, 0x0], [43])?;

            let expected_branch = Node::branch_with_two_children(
                0x1,
                Node::leaf([0x0], [42]),
                0x2,
                Node::leaf([0x0], [43]),
            );

            assert_eq!(expected_branch, updated_node);
            Ok(())
        }
    }

    #[cfg(test)]
    mod into_leaf {
        use super::*;

        #[test]
        fn duplicate_key() -> anyhow::Result<()> {
            let node = Node::leaf([0x0], [42]);
            let result = node.insert([0x0], [43]);
            assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
            Ok(())
        }

        #[test]
        fn empty_key() -> anyhow::Result<()> {
            let node = Node::leaf([0x0, 0x0], [42]);
            let updated_node = node.insert([], [43])?;

            let expected_branch =
                Node::branch_with_child_and_value(0, Entry::from(([0x0], [42])), [43]);

            assert_eq!(updated_node, expected_branch);
            Ok(())
        }

        #[test]
        fn non_empty_key() -> anyhow::Result<()> {
            let node = Node::leaf([0x0, 0x0], [42]);
            let updated_node = node.insert([0x0, 0x1], [43])?;

            let expected_child_node =
                Node::branch_with_two_children(0, Node::leaf([], [42]), 1, Node::leaf([], [43]));
            let expected_node = Node::extension([0x0], expected_child_node);

            assert_eq!(updated_node, expected_node);
            Ok(())
        }
    }

    #[cfg(test)]
    mod into_branch {
        use super::*;

        #[test]
        fn duplicate_key() {
            let node = Node::branch_with_value([42]);
            let result = node.insert([], [43]);
            assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
        }

        #[test]
        fn new_key() -> anyhow::Result<()> {
            let node = Node::branch_with_value([42]);
            let updated_node = node.insert([0x0, 0x0], [43])?;

            let expected_branch =
                Node::branch_with_child_and_value(0, Entry::from(([0x0], [43])), [42]);

            assert_eq!(expected_branch, updated_node);
            Ok(())
        }
    }

    #[cfg(test)]
    mod into_extension {
        use super::*;

        #[test]
        fn duplicate_key() {
            let node = Node::extension([0x0], Node::branch_with_value([42]));
            let result = node.insert([0x0], [43]);
            assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
        }

        #[test]
        fn new_key() -> anyhow::Result<()> {
            let extension = Node::extension([0x0, 0x0, 0x0], Node::leaf([], [42]));
            let node = extension.insert([0x0, 0x1], [43])?;

            let expected_child = Node::branch_with_two_children(
                0,
                Node::extension([0x0], Node::leaf([], [42])),
                1,
                Node::leaf([], [43]),
            );
            let expected_node = Node::extension([0x0], expected_child);

            assert_eq!(node, expected_node);
            Ok(())
        }
    }
}
