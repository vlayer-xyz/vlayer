#[cfg(test)]
mod insert {
    use crate::node::{constructors::EMPTY_CHILDREN, insert::entry::Entry, Node, NodeError};

    #[test]
    #[should_panic(expected = "Cannot insert into a digest node")]
    fn digest() {
        let node = Node::Digest(Default::default());
        node.insert([0x0], [42]).unwrap();
    }

    mod into_null {
        use super::*;

        #[test]
        fn empty_key() -> anyhow::Result<()> {
            let node = Node::Null;
            assert_eq!(node.insert([], [42])?, Node::branch_with_value([42]));
            Ok(())
        }

        #[test]
        fn short_key() -> anyhow::Result<()> {
            let node = Node::Null;
            assert_eq!(node.insert([0x0], [42])?, Node::leaf([0x0], [42]));
            Ok(())
        }

        #[test]
        fn long_key() -> anyhow::Result<()> {
            let node = Node::Null;
            assert_eq!(node.insert([0x0, 0x0], [42])?, Node::leaf([0x0, 0x0], [42]));
            Ok(())
        }

        #[test]
        fn double_insert() -> anyhow::Result<()> {
            let node = Node::Null;
            let updated_node = node.insert([0x1, 0x0], [42])?.insert([0x2, 0x0], [43])?;

            let mut children = EMPTY_CHILDREN.clone();
            children[0x1] = Some(Box::new(Node::leaf([0x0], [42])));
            children[0x2] = Some(Box::new(Node::leaf([0x0], [43])));
            let expected_branch = Node::Branch(children, None);

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

            let mut children = EMPTY_CHILDREN.clone();
            children[0] = Some(Box::new(Node::branch_with_value([42])));
            children[1] = Some(Box::new(Node::branch_with_value([43])));
            let expected_branch = Node::extension([0x0], Node::Branch(children, None));

            assert_eq!(updated_node, expected_branch);
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
        #[ignore]
        fn duplicate_key() {
            let node = Node::extension([0x0], Node::branch_with_value([42]));
            let result = node.insert([], [43]);

            assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
        }
    }
}
