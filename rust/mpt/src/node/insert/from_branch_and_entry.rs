use crate::node::Node;

use super::entry::Entry;

pub(crate) fn from_branch_and_entry(branch: Node, entry: Entry) -> Node {
    if let Node::Branch(children, branch_value) = branch {
        if entry.key.is_empty() {
            panic!("Key already exists");
        }
        let mut children = children;
        let (entry_first_nibble, remaining_entry) = entry.split_first_key_nibble();
        if let Some(existing_child) = children[entry_first_nibble as usize].take() {
            children[entry_first_nibble as usize] = Some(Box::new(
                existing_child.insert(remaining_entry.key, remaining_entry.value),
            ));
        } else {
            children[entry_first_nibble as usize] = Some(Box::new(remaining_entry.into()));
        }

        return Node::Branch(children, branch_value);
    } else {
        unreachable!("from_branch_and_entry is used only for Branch nodes");
    }
}

#[cfg(test)]
mod from_branch_and_entry {
    use crate::node::constructors::EMPTY_CHILDREN;

    use super::*;

    #[test]
    #[should_panic(expected = "Key already exists")]
    fn duplicate_key() {
        let branch = Node::branch(EMPTY_CHILDREN.clone(), Some([42]));
        from_branch_and_entry(branch, Entry::from(([], [43])));
    }

    #[test]
    fn branch_and_entry() {
        let branch = Node::branch(EMPTY_CHILDREN.clone(), Some([42]));
        let node = from_branch_and_entry(branch, ([0x0], [43]).into());

        let mut children = EMPTY_CHILDREN.clone();
        children[0] = Some(Box::new(Entry::from(([], [43])).into()));
        let expected_node = Node::Branch(children, Some([42].into()));

        assert_eq!(node, expected_node);
    }

    #[test]
    fn creating_branch_with_nested_descendants() {
        let mut children = EMPTY_CHILDREN.clone();
        children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([]))));
        let branch = Node::branch(children, Some([]));

        let node = from_branch_and_entry(branch, ([0x0, 0x0], [42]).into());

        let mut expected_node_children = EMPTY_CHILDREN.clone();
        let mut internal_node_children = EMPTY_CHILDREN.clone();
        internal_node_children[0] = Some(Box::new(Entry::from(([], [42])).into()));
        expected_node_children[0] = Some(Box::new(Node::branch(internal_node_children, Some([]))));
        let expected_node = Node::branch(expected_node_children, Some([]));

        assert_eq!(node, expected_node);
    }
}
