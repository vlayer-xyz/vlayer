use nybbles::Nibbles;

use crate::node::{Node, NodeError};

use self::extract_common_prefix::extract_common_prefix;
use self::from_extension_and_entry_no_common_prefix::from_extension_and_entry_no_common_prefix;
use super::entry::Entry;

mod extract_common_prefix;
mod from_extension_and_entry_no_common_prefix;

impl Node {
    pub(crate) fn insert_entry_into_extension(
        self,
        entry: impl Into<Entry>,
    ) -> Result<Node, NodeError> {
        let Node::Extension(key, extension_node) = self else {
            unreachable!("from_extension_and_entry is used only for Extension nodes");
        };

        let entry = entry.into();

        let (common_prefix, remaining_extension_key, remaining_entry_key) =
            extract_common_prefix((*key).as_ref(), entry.key.as_ref());

        if common_prefix.is_empty() {
            // Because common_prefix is empty we know that remaining_extension_key is non empty: Extension
            // always has non empty key. Hence we can create Extension with non empty key from remaining_extension_key.
            return Ok(from_extension_and_entry_no_common_prefix(
                Node::extension(remaining_extension_key, *extension_node),
                entry,
            ));
        }

        if remaining_extension_key.is_empty() {
            let extension_node =
                extension_node.insert(Nibbles::from_nibbles(remaining_entry_key), entry.value)?;
            return Ok(Node::extension(common_prefix, extension_node));
        }

        let extension_node = from_extension_and_entry_no_common_prefix(
            Node::extension(remaining_extension_key, *extension_node),
            (remaining_entry_key, entry.value).into(),
        );

        Ok(Node::extension(common_prefix, extension_node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::constructors::EMPTY_CHILDREN;

    #[test]
    #[should_panic(expected = "from_extension_and_entry is used only for Extension nodes")]
    fn unreachable() {
        let node = Node::Null;
        let entry = ([0x0], [42]);
        let _ = node.insert_entry_into_extension(entry);
    }

    mod common_prefix_empty {
        use super::*;

        #[test]
        fn one_nibble_extension() {
            let extension = Node::extension([0x0], Node::branch_with_value([42]));
            let node = extension.insert_entry_into_extension(([], [43])).unwrap();

            let child = Node::branch_with_value([42]);
            let expected_node = Node::branch_with_child_and_value(0, child, [43]);

            assert_eq!(node, expected_node);
        }

        #[test]
        fn multiple_nibbles_extension() {
            let extension = Node::extension([0x0, 0x0], Node::branch_with_value([42]));
            let node = extension.insert_entry_into_extension(([], [43])).unwrap();

            let child = Node::extension([0x0], Node::branch_with_value([42]));
            let expected_node = Node::branch_with_child_and_value(0, child, [43]);

            assert_eq!(node, expected_node);
        }
    }

    mod common_prefix_non_empty {
        use super::*;

        mod remaining_extension_key_empty {
            use super::*;

            #[test]
            fn single_nibble_entry_key() {
                let extension = Node::extension([0x0], Node::branch_with_value([42]));
                let result = extension.insert_entry_into_extension(([0x0], [43]));
                assert_eq!(result.unwrap_err(), NodeError::DuplicatedKey);
            }

            #[test]
            fn multiple_nibbles_entry_key() {
                let extension = Node::extension([0x0], Node::branch_with_value([42]));
                let node = extension
                    .insert_entry_into_extension(([0x0, 0x0], [43]))
                    .unwrap();

                let child = Node::branch_with_value([43]);
                let expected_node =
                    Node::extension([0x0], Node::branch_with_child_and_value(0, child, [42]));

                assert_eq!(node, expected_node);
            }
        }

        mod remaining_extension_key_non_empty {
            use super::*;

            #[test]
            fn single_nibble_remaining_extension_key() {
                let extension = Node::extension([0x0, 0x0], Node::branch_with_value([42]));
                let node = extension
                    .insert_entry_into_extension(([0x0, 0x1], [43]))
                    .unwrap();

                let mut children = EMPTY_CHILDREN.clone();
                children[0] = Some(Box::new(Node::branch_with_value([42])));
                children[1] = Some(Box::new(Node::branch_with_value([43])));
                let expected_node = Node::extension([0x0], Node::Branch(children, None));

                assert_eq!(node, expected_node);
            }

            #[test]
            fn multiple_nibbles_remaining_extension_key() {
                let extension = Node::extension([0x0, 0x0, 0x0], Node::branch_with_value([42]));
                let node = extension
                    .insert_entry_into_extension(([0x0, 0x1], [43]))
                    .unwrap();

                let mut children = EMPTY_CHILDREN.clone();
                children[0] = Some(Box::new(Node::extension(
                    [0x0],
                    Node::branch_with_value([42]),
                )));
                children[1] = Some(Box::new(Node::branch_with_value([43])));
                let expected_node = Node::extension([0x0], Node::Branch(children, None));

                assert_eq!(node, expected_node);
            }
        }
    }
}
