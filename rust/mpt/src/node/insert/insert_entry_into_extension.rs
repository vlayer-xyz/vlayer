use crate::{
    key_nibbles::KeyNibbles,
    node::{Node, NodeError},
};

use self::extract_common_prefix::extract_common_prefix;
use self::from_extension_and_entry_no_common_prefix::from_extension_and_entry_no_common_prefix;
use super::entry::Entry;

mod extract_common_prefix;
mod from_extension_and_entry_no_common_prefix;

impl Node {
    pub(crate) fn insert_entry_into_extension(self, entry: Entry) -> Result<Node, NodeError> {
        let Node::Extension(key, extension_node) = self else {
            unreachable!("from_extension_and_entry is used only for Extension nodes");
        };

        let (common_prefix, remaining_extension_key, remaining_entry_key) =
            extract_common_prefix((*key).clone(), entry.key.clone());

        if common_prefix.is_empty() {
            // Because common_prefix is empty we know that remaining_extension_key is non empty: Extension
            // always has non empty key. Hence we can create Extension with non empty key from remaining_extension_key.
            return Ok(from_extension_and_entry_no_common_prefix(
                Node::Extension(
                    KeyNibbles::from_nibbles(remaining_extension_key),
                    extension_node,
                ),
                entry,
            ));
        }

        if remaining_extension_key.is_empty() {
            return Ok(Node::Extension(
                KeyNibbles::from_nibbles(common_prefix),
                Box::new(extension_node.insert(remaining_entry_key, entry.value)?),
            ));
        }

        Ok(Node::Extension(
            KeyNibbles::from_nibbles(common_prefix),
            Box::new(from_extension_and_entry_no_common_prefix(
                Node::Extension(
                    KeyNibbles::from_nibbles(remaining_extension_key),
                    extension_node,
                ),
                Entry {
                    key: remaining_entry_key,
                    value: entry.value,
                },
            )),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::constructors::EMPTY_CHILDREN;

    #[test]
    #[should_panic(expected = "from_extension_and_entry is used only for Extension nodes")]
    fn unreachable() {
        let node = Node::Branch(EMPTY_CHILDREN.clone(), None);
        let entry = ([0x0], [42]).into();
        let _ = node.insert_entry_into_extension(entry);
    }

    mod common_prefix_empty {
        use super::*;

        #[test]
        fn one_nibble_extension() {
            let extension = Node::extension([0x0], Node::branch_with_value([42]));
            let node = extension
                .insert_entry_into_extension(([], [43]).into())
                .unwrap();

            let child = Node::branch_with_value([42]);
            let expected_node = Node::branch_with_child_and_value(0, child, [43]);

            assert_eq!(node, expected_node);
        }

        #[test]
        fn multiple_nibbles_extension() {
            let extension = Node::extension([0x0, 0x0], Node::branch_with_value([42]));
            let node = extension
                .insert_entry_into_extension(([], [43]).into())
                .unwrap();

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
                let result = extension.insert_entry_into_extension(([0x0], [43]).into());
                assert_eq!(result.unwrap_err(), NodeError::DuplicatedKey);
            }

            #[test]
            fn multiple_nibbles_entry_key() {
                let extension = Node::extension([0x0], Node::branch_with_value([42]));
                let node = extension
                    .insert_entry_into_extension(([0x0, 0x0], [43]).into())
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
                    .insert_entry_into_extension(([0x0, 0x1], [43]).into())
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
                    .insert_entry_into_extension(([0x0, 0x1], [43]).into())
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
