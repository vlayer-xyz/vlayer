use crate::node::{constructors::EMPTY_CHILDREN, insert::entry::Entry, Node};

pub(crate) fn from_extension_and_entry_no_common_prefix(extension: Node, entry: Entry) -> Node {
    let Node::Extension(key, extension_node) = extension else {
        unreachable!("from_extension_and_entry_no_common_prefix is used only for Extension nodes");
    };

    let (first_extension_nibble, remaining_extension_key) = key.split_first().unwrap();
    if entry.key.is_empty() {
        let mut children = EMPTY_CHILDREN.clone();

        insert_extension_into_branch_child(
            &mut children,
            *first_extension_nibble,
            remaining_extension_key,
            *extension_node,
        );

        return Node::branch_with_children_and_value(children, entry.value);
    }

    let (entry_first_nibble, remaining_entry) = entry.split_first_key_nibble();

    let mut children = EMPTY_CHILDREN.clone();
    children[entry_first_nibble as usize] = Some(Box::new(remaining_entry.into()));

    // Extension and entry have no common prefix so children[entry_first_nibble] won't be overwritten.
    insert_extension_into_branch_child(
        &mut children,
        *first_extension_nibble,
        remaining_extension_key,
        *extension_node,
    );

    Node::Branch(children, None)
}

// Depending on extension key length either extension_node or
// Node::extension(remaining_extension_key, extension_node) is inserted.
fn insert_extension_into_branch_child(
    children: &mut [Option<Box<Node>>; 16],
    first_extension_nibble: u8,
    remaining_extension_key: &[u8],
    extension_node: Node,
) {
    if remaining_extension_key.is_empty() {
        children[first_extension_nibble as usize] = Some(Box::new(extension_node));
    } else {
        children[first_extension_nibble as usize] = Some(Box::new(Node::extension(
            remaining_extension_key,
            extension_node,
        )));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{constructors::EMPTY_CHILDREN, insert::entry::Entry, Node};

    mod empty_entry_key {
        use super::*;

        #[test]
        fn one_nibble_extension() {
            let extension = Node::extension([0x0], Node::branch_with_value([42]));
            let entry: Entry = ([], [43]).into();
            let node = from_extension_and_entry_no_common_prefix(extension, entry);
            let child = Node::branch_with_value([42]);
            let expected_node = Node::branch_with_child_and_value(0, child, [43]);

            assert_eq!(node, expected_node);
        }

        #[test]
        fn multiple_nibbles_extension() {
            let extension = Node::extension([0x0, 0x0], Node::branch_with_value([42]));
            let entry: Entry = ([], [43]).into();

            let node = from_extension_and_entry_no_common_prefix(extension, entry);

            let child = Node::extension([0x0], Node::branch_with_value([42]));
            let expected_node = Node::branch_with_child_and_value(0, child, [43]);

            assert_eq!(node, expected_node);
        }
    }

    mod non_empty_entry_key {
        use super::*;

        #[test]
        fn one_nibble_extension() {
            let extension = Node::extension([0x0], Node::branch_with_value([42]));
            let node = extension
                .insert_entry_into_extension(([0x1], [43]).into())
                .unwrap();

            let mut children = EMPTY_CHILDREN.clone();
            children[0] = Some(Box::new(Node::branch_with_value([42])));
            children[1] = Some(Box::new(Node::branch_with_value([43])));
            let expected_node = Node::Branch(children, None);

            assert_eq!(node, expected_node);
        }

        #[test]
        fn multiple_nibbles_extension() {
            let extension = Node::extension([0x0, 0x0], Node::branch_with_value([42]));
            let node = extension
                .insert_entry_into_extension(([0x1], [43]).into())
                .unwrap();

            let mut children = EMPTY_CHILDREN.clone();
            children[0] = Some(Box::new(Node::extension(
                [0x0],
                Node::branch_with_value([42]),
            )));
            children[1] = Some(Box::new(Node::branch_with_value([43])));
            let expected_node = Node::Branch(children, None);

            assert_eq!(node, expected_node);
        }
    }
}

#[cfg(test)]
mod insert_extension_into_branch_child {
    use super::*;
    use crate::node::constructors::EMPTY_CHILDREN;

    #[test]
    fn no_remaining_extension_key() {
        let mut children = EMPTY_CHILDREN.clone();
        let extension_node = Node::branch_with_value([42]);

        insert_extension_into_branch_child(&mut children, 0x0, &[], extension_node);

        let expected_children = {
            let mut children = EMPTY_CHILDREN.clone();
            children[0] = Some(Box::new(Node::branch_with_value([42])));
            children
        };

        assert_eq!(children, expected_children);
    }

    #[test]
    fn remaining_extension_key() {
        let mut children = EMPTY_CHILDREN.clone();
        let extension_node = Node::branch_with_value([42]);

        insert_extension_into_branch_child(&mut children, 0x0, &[0x0], extension_node);

        let expected_children = {
            let mut children = EMPTY_CHILDREN.clone();
            children[0] = Some(Box::new(Node::extension(
                [0x0],
                Node::branch_with_value([42]),
            )));
            children
        };

        assert_eq!(children, expected_children);
    }
}
