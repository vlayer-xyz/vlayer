use crate::node::Node;

use super::entry::Entry;

pub(crate) fn from_branch_and_entry(branch: Node, entry: Entry) -> Result<Node, String> {
    if let Node::Branch(mut children, branch_value) = branch {
        if entry.key.is_empty() {
            if branch_value.is_some() {
                return Err("Key already exists".to_string());
            } else {
                return Ok(Node::Branch(children, Some(entry.value)));
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

        Ok(Node::Branch(children, branch_value))
    } else {
        unreachable!("from_branch_and_entry is used only for Branch nodes");
    }
}

#[cfg(test)]
mod tests {
    use crate::node::constructors::EMPTY_CHILDREN;

    use super::*;

    #[test]
    fn duplicate_key() {
        let branch = Node::branch(EMPTY_CHILDREN.clone(), Some([42]));
        let result = from_branch_and_entry(branch, Entry::from(([], [43])));
        assert!(result.is_err(), "Expected an error, but got Ok");
        assert_eq!(result.unwrap_err(), "Key already exists");
    }

    #[test]
    fn add_value_to_branch_without_value() {
        let branch = Node::Branch(EMPTY_CHILDREN.clone(), None);
        let node = from_branch_and_entry(branch, ([], [42]).into()).unwrap();

        let expected_node = Node::branch(EMPTY_CHILDREN.clone(), Some([42]));

        assert_eq!(node, expected_node);
    }

    #[test]
    fn branch_with_no_children() {
        let branch = Node::branch(EMPTY_CHILDREN.clone(), Some([42]));
        let node = from_branch_and_entry(branch, ([0x0], [43]).into()).unwrap();

        let mut children = EMPTY_CHILDREN.clone();
        children[0] = Some(Box::new(Entry::from(([], [43])).into()));
        let expected_node = Node::Branch(children, Some([42].into()));

        assert_eq!(node, expected_node);
    }

    #[test]
    fn branch_with_children() {
        let mut children = EMPTY_CHILDREN.clone();
        children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([]))));
        let branch = Node::branch(children, Some([]));

        let node = from_branch_and_entry(branch, ([0x0, 0x0], [42]).into()).unwrap();

        let mut expected_node_children = EMPTY_CHILDREN.clone();
        let mut internal_node_children = EMPTY_CHILDREN.clone();
        internal_node_children[0] = Some(Box::new(Entry::from(([], [42])).into()));
        expected_node_children[0] = Some(Box::new(Node::branch(internal_node_children, Some([]))));
        let expected_node = Node::branch(expected_node_children, Some([]));

        assert_eq!(node, expected_node);
    }
}
