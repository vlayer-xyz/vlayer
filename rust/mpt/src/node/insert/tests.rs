#[cfg(test)]
mod insert {
    use nybbles::Nibbles;

    use crate::node::{constructors::EMPTY_CHILDREN, Node};

    #[test]
    #[should_panic(expected = "Cannot insert into a digest node")]
    fn digest() {
        let node = Node::Digest(Default::default());
        node.insert(Nibbles::unpack([0x0]), [42]).unwrap();
    }

    mod into_null {
        use super::*;

        #[test]
        fn empty_key() {
            let node = Node::Null;
            assert_eq!(
                node.insert(Nibbles::unpack([]), [42]).unwrap(),
                Node::branch_with_value(EMPTY_CHILDREN.clone(), [42])
            );
        }

        #[test]
        fn short_key() {
            let node = Node::Null;
            assert_eq!(
                node.insert(Nibbles::unpack([0x0]), [42]).unwrap(),
                Node::leaf([0x0, 0x0], [42])
            );
        }

        #[test]
        fn long_key() {
            let node = Node::Null;
            assert_eq!(
                node.insert(Nibbles::unpack([0x0, 0x0]), [42]).unwrap(),
                //here we create a leaf from nibbles, not from bytes as above
                Node::leaf([0x0, 0x0, 0x0, 0x0], [42])
            );
        }

        #[test]
        fn double_insert() {
            let node = Node::Null;
            let updated_node = node
                .insert(Nibbles::unpack([0x10]), [42])
                .unwrap()
                .insert(Nibbles::unpack([0x20]), [43])
                .unwrap();

            let mut children = EMPTY_CHILDREN.clone();
            children[0x1] = Some(Box::new(Node::leaf([0x0], [42])));
            children[0x2] = Some(Box::new(Node::leaf([0x0], [43])));
            let expected_branch = Node::Branch(children, None);

            assert_eq!(expected_branch, updated_node);
        }
    }

    #[cfg(test)]
    mod into_leaf {
        use crate::node::{insert::entry::Entry, NodeError};

        use super::*;

        #[test]
        fn duplicate_key() {
            // Trie::insert always inserts even number of nibbles so two is the minimal
            // number of nibbles for a leaf, cause leafs cannot have empty keys.
            let node = Node::leaf([0x0, 0x0], [42]);
            let result = node.insert(Nibbles::unpack([0x0]), [43]);
            assert_eq!(result.unwrap_err(), NodeError::DuplicatedKey);
        }

        #[test]
        fn empty_key() {
            let node = Node::leaf([0x0, 0x0], [42]);
            let updated_node = node.insert(Nibbles::unpack([]), [43]).unwrap();

            let mut children = EMPTY_CHILDREN.clone();
            children[0] = Some(Box::new(Entry::from(([0x0], [42])).into()));
            let expected_branch = Node::branch_with_value(children, [43]);

            assert_eq!(updated_node, expected_branch);
        }

        #[test]
        fn non_empty_key() {
            let node = Node::leaf([0x0, 0x0], [42]);
            let updated_node = node.insert(Nibbles::unpack([0x1]), [43]).unwrap();

            let mut children = EMPTY_CHILDREN.clone();
            children[0] = Some(Box::new(Node::branch_with_value(
                EMPTY_CHILDREN.clone(),
                [42],
            )));
            children[1] = Some(Box::new(Node::branch_with_value(
                EMPTY_CHILDREN.clone(),
                [43],
            )));
            let expected_branch = Node::extension([0x0], Node::Branch(children, None));

            assert_eq!(updated_node, expected_branch);
        }
    }

    #[cfg(test)]
    mod into_branch {
        use crate::node::{constructors::EMPTY_CHILDREN, insert::entry::Entry, NodeError};

        use super::*;

        #[test]
        fn duplicate_key() {
            let node = Node::branch_with_value(EMPTY_CHILDREN.clone(), [42]);
            let result = node.insert(Nibbles::unpack([]), [43]);
            assert_eq!(result.unwrap_err(), NodeError::DuplicatedKey);
        }

        #[test]
        fn new_key() {
            let node = Node::branch_with_value(EMPTY_CHILDREN.clone(), [42]);
            let updated_node = node.insert(Nibbles::unpack([0x0]), [43]).unwrap();

            let mut expected_branch = Node::branch_with_value(EMPTY_CHILDREN.clone(), [42]);
            if let Node::Branch(ref mut children, _) = expected_branch {
                children[0] = Some(Box::new(Entry::from(([0x0], [43])).into()));
            }

            assert_eq!(expected_branch, updated_node);
        }
    }
}
