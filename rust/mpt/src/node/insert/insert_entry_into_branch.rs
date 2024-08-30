use crate::node::{Node, NodeError};

use super::entry::Entry;

impl Node {
    pub(crate) fn insert_entry_into_branch(
        self,
        entry: impl Into<Entry>,
    ) -> Result<Node, NodeError> {
        let Node::Branch(mut children, branch_value) = self else {
            unreachable!("insert_entry_into_branch is used only for Branch nodes");
        };

        let entry = entry.into();
        if entry.key.is_empty() {
            if branch_value.is_some() {
                return Err(NodeError::DuplicateKey);
            }
            return Ok(Node::Branch(children, Some(entry.value)));
        }

        let (entry_first_nibble, remaining_entry) = entry.split_first_key_nibble();

        if let Some(existing_child) = children[entry_first_nibble as usize].take() {
            children[entry_first_nibble as usize] = Some(Box::new(
                existing_child.insert(&*remaining_entry.key, remaining_entry.value)?,
            ));
        } else {
            children[entry_first_nibble as usize] = Some(Box::new(remaining_entry.into()));
        }

        Ok(Node::Branch(children, branch_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::constructors::EMPTY_CHILD;

    pub static EMPTY_BRANCH: Node = Node::Branch([EMPTY_CHILD; 16], None);

    #[test]
    #[should_panic(expected = "insert_entry_into_branch is used only for Branch nodes")]
    fn unreachable() {
        let leaf = Node::leaf([0x0], [42]);
        leaf.insert_entry_into_branch(([0x0], [42])).unwrap();
    }

    mod empty_key {
        use super::*;

        #[test]
        fn branch_value_none() -> anyhow::Result<()> {
            let branch = EMPTY_BRANCH.clone();
            let node = branch.insert_entry_into_branch(([], [42]))?;

            let expected_node = Node::branch_with_value([42]);

            assert_eq!(node, expected_node);
            Ok(())
        }

        #[test]
        fn branch_value_some() {
            let branch = Node::branch_with_value([42]);
            let result = branch.insert_entry_into_branch(([], [43]));
            assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
        }
    }

    mod non_empty_key {
        use super::*;

        mod child_none {
            use super::*;

            #[test]
            fn no_nibble_remaining() -> anyhow::Result<()> {
                let branch = EMPTY_BRANCH.clone();
                let node = branch.insert_entry_into_branch(([0x0], [42]))?;

                let expected_node = Node::branch_with_child(0, Node::branch_with_value([42]));

                assert_eq!(node, expected_node);
                Ok(())
            }

            #[test]
            fn nibble_remaining() -> anyhow::Result<()> {
                let branch = EMPTY_BRANCH.clone();
                let node = branch.insert_entry_into_branch(([0x0, 0x0], [42]))?;

                let expected_node = Node::branch_with_child(0, Node::leaf([0x0], [42]));

                assert_eq!(node, expected_node);
                Ok(())
            }
        }

        mod child_some {
            use super::*;

            #[test]
            fn no_nibble_remaining() {
                let branch = Node::branch_with_child(0, Node::branch_with_value([42]));
                let result = branch.insert_entry_into_branch(([0x0], [43]));
                assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
            }

            #[test]
            fn nibble_remaining() -> anyhow::Result<()> {
                let branch = Node::branch_with_child_and_value(0, Node::branch_with_value([]), []);

                let node = branch.insert_entry_into_branch(([0x0, 0x0], [42]))?;

                let expected_node = Node::branch_with_child_and_value(
                    0,
                    Node::branch_with_child_and_value(0, Entry::from(([], [42])), []),
                    [],
                );

                assert_eq!(node, expected_node);
                Ok(())
            }
        }
    }
}
