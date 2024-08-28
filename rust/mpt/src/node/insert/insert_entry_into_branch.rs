use crate::node::{MPTError, Node};

use super::entry::Entry;

impl Node {
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn insert_entry_into_branch(self, entry: Entry) -> Result<Node, MPTError> {
        if let Node::Branch(children, branch_value) = self {
            let mut children = children;
            if entry.key.is_empty() {
                if branch_value.is_some() {
                    return Err(MPTError::DuplicatedKey(
                        String::from_utf8(entry.key.to_vec()).expect("Invalid UTF-8"),
                    ));
                } else {
                    return Ok(Node::Branch(children.clone(), Some(entry.value)));
                }
            }

            let (entry_first_nibble, remaining_entry) = entry.split_first_key_nibble();

            if let Some(existing_child) = children[entry_first_nibble as usize].take() {
                children[entry_first_nibble as usize] = Some(Box::new(
                    existing_child.insert(remaining_entry.key, remaining_entry.value),
                ));
            } else {
                children[entry_first_nibble as usize] = Some(Box::new(remaining_entry.into()));
            }

            Ok(Node::Branch(children.clone(), branch_value.clone()))
        } else {
            unreachable!("insert_entry_into_branch is used only for Branch nodes");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::constructors::EMPTY_CHILDREN;

    use super::*;

    #[test]
    #[should_panic(expected = "insert_entry_into_branch is used only for Branch nodes")]
    fn unreachable() {
        let leaf = Node::leaf([0x0], [42]);
        let _ = leaf.insert_entry_into_branch(([0x0], [42]).into());
    }

    mod empty_key {
        use super::*;

        #[test]
        fn branch_value_none() {
            let branch = Node::Branch(EMPTY_CHILDREN.clone(), None);
            let node = branch.insert_entry_into_branch(([], [42]).into()).unwrap();

            let expected_node = Node::branch(EMPTY_CHILDREN.clone(), Some([42]));

            assert_eq!(node, expected_node);
        }

        #[test]
        fn branch_value_some() {
            let branch = Node::branch(EMPTY_CHILDREN.clone(), Some([42]));
            let result = branch.insert_entry_into_branch(([], [43]).into());
            assert!(result.is_err(), "Expected an error, but got Ok");
            assert_eq!(result.unwrap_err(), MPTError::DuplicatedKey("".to_string()));
        }
    }

    mod non_empty_key {
        use super::*;

        mod child_none {
            use super::*;

            #[test]
            fn no_nibble_remaining() {
                let branch = Node::Branch(EMPTY_CHILDREN.clone(), None);
                let node = branch
                    .insert_entry_into_branch(([0x0], [42]).into())
                    .unwrap();

                let mut children = EMPTY_CHILDREN.clone();
                children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([42]))));
                let expected_node = Node::Branch(children, None);

                assert_eq!(node, expected_node);
            }

            #[test]
            fn nibble_remaining() {
                let branch = Node::Branch(EMPTY_CHILDREN.clone(), None);
                let node = branch
                    .insert_entry_into_branch(([0x0, 0x0], [42]).into())
                    .unwrap();

                let mut children = EMPTY_CHILDREN.clone();
                children[0] = Some(Box::new(Node::leaf([0x0], [42])));
                let expected_node = Node::Branch(children, None);

                assert_eq!(node, expected_node);
            }
        }

        mod child_some {
            use super::*;

            #[test]
            #[should_panic(expected = "DuplicatedKey(\"\")")]
            fn no_nibble_remaining() {
                let mut children = EMPTY_CHILDREN.clone();
                children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([42]))));
                let branch = Node::Branch(children, None);
                branch
                    .insert_entry_into_branch(([0x0], [43]).into())
                    .unwrap();
            }

            #[test]
            fn nibble_remaining() {
                let mut children = EMPTY_CHILDREN.clone();
                children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([]))));
                let branch = Node::branch(children, Some([]));

                let node = branch
                    .insert_entry_into_branch(([0x0, 0x0], [42]).into())
                    .unwrap();

                let mut expected_node_children = EMPTY_CHILDREN.clone();
                let mut internal_node_children = EMPTY_CHILDREN.clone();
                internal_node_children[0] = Some(Box::new(Entry::from(([], [42])).into()));
                expected_node_children[0] =
                    Some(Box::new(Node::branch(internal_node_children, Some([]))));
                let expected_node = Node::branch(expected_node_children, Some([]));

                assert_eq!(node, expected_node);
            }
        }
    }
}
