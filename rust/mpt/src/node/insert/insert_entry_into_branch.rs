use super::entry::Entry;
use crate::node::{Node, NodeError};

impl<D> Node<D> {
    pub(crate) fn insert_entry_into_branch(
        self,
        entry: impl Into<Entry>,
    ) -> Result<Node<D>, NodeError> {
        let Node::Branch(mut children, branch_value) = self else {
            unreachable!("insert_entry_into_branch is used only for Branch nodes");
        };

        let entry = entry.into();
        if entry.key.is_empty() {
            if branch_value.is_some() {
                return Err(NodeError::DuplicateKey);
            }
            // ![Entry key empty](../../../images/into_branch_0.png)
            return Ok(Node::Branch(children, Some(entry.value)));
        }

        let (entry_first_nibble, remaining_entry) = entry.split_first_key_nibble();

        let child = children[entry_first_nibble as usize].take();
        let new_child = insert_entry_into_child(child, remaining_entry)?;
        children[entry_first_nibble as usize] = Some(Box::new(new_child));

        Ok(Node::Branch(children, branch_value))
    }
}

fn insert_entry_into_child<D>(
    child: Option<Box<Node<D>>>,
    entry: Entry,
) -> Result<Node<D>, NodeError> {
    // Depending on the child node, either insert the entry into the child node or create a new child node
    // ![Into existing child](../../../images/into_branch_1.png)
    // ![Into empty child](../../../images/into_branch_2.png)
    match child {
        Some(existing_child) => existing_child.insert(&*entry.key, entry.value),
        None => Ok(entry.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KeccakNode as Node;

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
            let branch = Node::empty_branch();
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
                let branch = Node::empty_branch();
                let node = branch.insert_entry_into_branch(([0x0], [42]))?;

                let expected_node = Node::branch_with_child(0, Node::leaf([], [42]));

                assert_eq!(node, expected_node);
                Ok(())
            }

            #[test]
            fn nibble_remaining() -> anyhow::Result<()> {
                let branch = Node::empty_branch();
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
                let branch =
                    Node::branch_with_child_and_value(0, Node::branch_with_value([0]), [0]);

                let node = branch.insert_entry_into_branch(([0x0, 0x0], [42]))?;

                let expected_node = Node::branch_with_child_and_value(
                    0,
                    Node::branch_with_child_and_value(0, Entry::from(([], [42])), [0]),
                    [0],
                );

                assert_eq!(node, expected_node);
                Ok(())
            }
        }
    }
}
