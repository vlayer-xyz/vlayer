use crate::node::{Node, NodeError};

use super::entry::Entry;

impl Node {
    pub(crate) fn insert_entry_into_branch(self, entry: Entry) -> Result<Node, NodeError> {
        let Node::Branch(children, branch_value) = self else {
            unreachable!("insert_entry_into_branch is used only for Branch nodes");
        };

        let mut children = children;
        if entry.key.is_empty() {
            if branch_value.is_some() {
                return Err(NodeError::DuplicatedKey);
            }
            return Ok(Node::Branch(children, Some(entry.value)));
        }

        let (entry_first_nibble, remaining_entry) = entry.split_first_key_nibble();

        if let Some(existing_child) = children[entry_first_nibble as usize].take() {
            children[entry_first_nibble as usize] = Some(Box::new(
                existing_child.insert(remaining_entry.key, remaining_entry.value)?,
            ));
        } else {
            children[entry_first_nibble as usize] = Some(Box::new(remaining_entry.into()));
        }

        Ok(Node::Branch(children, branch_value))
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
        leaf.insert_entry_into_branch(([0x0], [42]).into()).unwrap();
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
            assert_eq!(result.unwrap_err(), NodeError::DuplicatedKey);
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
            fn no_nibble_remaining() {
                let mut children = EMPTY_CHILDREN.clone();
                children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([42]))));
                let branch = Node::Branch(children, None);
                let result = branch.insert_entry_into_branch(([0x0], [43]).into());
                assert_eq!(result.unwrap_err(), NodeError::DuplicatedKey);
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
