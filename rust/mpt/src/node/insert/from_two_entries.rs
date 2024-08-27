use crate::node::{constructors::EMPTY_CHILDREN, MPTError, Node};

use super::entry::Entry;

fn order_entries(lhs: Entry, rhs: Entry) -> (Entry, Entry) {
    if lhs.key.len() <= rhs.key.len() {
        (lhs, rhs)
    } else {
        (rhs, lhs)
    }
}

pub(crate) fn from_two_entries(lhs: Entry, rhs: Entry) -> Result<Node, MPTError> {
    let (shorter, longer) = order_entries(lhs, rhs);
    if shorter.key == longer.key {
        return Err(MPTError::DuplicatedKey(
            String::from_utf8(shorter.key.to_vec()).expect("Invalid UTF-8"),
        ));
    }

    if shorter.key.is_empty() {
        let (longer_first_nibble, remaining_longer) = longer.split_first_key_nibble();
        let mut children = EMPTY_CHILDREN.clone();
        children[longer_first_nibble as usize] = Some(Box::new(remaining_longer.into()));

        return Ok(Node::Branch(children, Some(shorter.value)));
    }
    let (shorter_first_nibble, remaining_shorter) = shorter.split_first_key_nibble();
    let (longer_first_nibble, remaining_longer) = longer.split_first_key_nibble();

    if shorter_first_nibble != longer_first_nibble {
        let mut children = EMPTY_CHILDREN.clone();
        children[shorter_first_nibble as usize] = Some(Box::new(remaining_shorter.into()));
        children[longer_first_nibble as usize] = Some(Box::new(remaining_longer.into()));

        return Ok(Node::Branch(children, None));
    }

    let node = from_two_entries(remaining_shorter, remaining_longer)?;

    let result_node = match node {
        Node::Branch(_, _) => Node::extension([shorter_first_nibble], node),
        Node::Extension(nibbles, child) => {
            Node::Extension(nibbles.push_front(shorter_first_nibble), child)
        }
        _ => unreachable!("from_two_ordered_entries should return only Branch or Extension"),
    };

    Ok(result_node)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "DuplicatedKey(\"\")")]
    fn two_empty_keys() {
        let first_entry: Entry = ([], [42]).into();
        let second_entry: Entry = ([], [43]).into();

        let _ = from_two_entries(first_entry, second_entry).unwrap();
    }

    #[test]
    fn one_empty_key() {
        let first_entry: Entry = ([], [42]).into();
        let second_entry: Entry = ([0x0], [43]).into();
        let node = from_two_entries(first_entry, second_entry).unwrap();

        let mut children = EMPTY_CHILDREN.clone();
        children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([43]))));
        let expected_node = Node::branch(children, Some([42]));

        assert_eq!(node, expected_node);
    }

    #[test]
    fn duplicate_key() {
        let old_entry: Entry = ([0], [42]).into();
        let entry: Entry = ([0], [43]).into();
        let result = from_two_entries(old_entry, entry);
        assert!(result.is_err(), "Expected an error, but got Ok");
        assert_eq!(
            result.unwrap_err(),
            MPTError::DuplicatedKey("\0".to_string())
        );
    }

    #[test]
    fn different_single_nibbles() {
        let first_entry: Entry = ([0x0], [42]).into();
        let second_entry: Entry = ([0x1], [43]).into();

        let node = from_two_entries(first_entry, second_entry).unwrap();

        let mut children = EMPTY_CHILDREN.clone();
        children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([42]))));
        children[1] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([43]))));
        let expected_node = Node::Branch(children, None);

        assert_eq!(node, expected_node);
    }

    #[test]
    fn no_common_prefix() {
        let first_entry: Entry = ([0x0, 0x0], [42]).into();
        let second_entry: Entry = ([0x1, 0x0], [43]).into();
        let node = from_two_entries(first_entry, second_entry).unwrap();

        let mut children = EMPTY_CHILDREN.clone();
        children[0] = Some(Box::new(Node::leaf([0], [42])));
        children[1] = Some(Box::new(Node::leaf([0], [43])));
        let expected_node = Node::Branch(children, None);

        assert_eq!(node, expected_node);
    }

    #[test]
    fn common_prefix() {
        let first_entry: Entry = ([0x0, 0x0], [42]).into();
        let second_entry: Entry = ([0x0, 0x1], [43]).into();

        let node = from_two_entries(first_entry, second_entry).unwrap();

        let mut children = EMPTY_CHILDREN.clone();
        children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([42]))));
        children[1] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([43]))));
        let expected_node_child = Node::Branch(children, None);
        let expected_node = Node::extension([0x0], expected_node_child);

        assert_eq!(node, expected_node);
    }

    #[test]
    fn long_common_prefix() {
        let first_entry: Entry = ([0x0, 0x1, 0x0], [42]).into();
        let second_entry: Entry = ([0x0, 0x1, 0x1], [43]).into();

        let node = from_two_entries(first_entry, second_entry).unwrap();

        let mut children = EMPTY_CHILDREN.clone();
        children[0] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([42]))));
        children[1] = Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([43]))));
        let expected_node_child = Node::Branch(children, None);
        let expected_node = Node::extension([0x0, 0x1], expected_node_child);

        assert_eq!(node, expected_node);
    }

    #[test]
    fn common_prefix_with_different_long_suffix() {
        let first_entry: Entry = ([0x0, 0x0, 0x1], [42]).into();
        let second_entry: Entry = ([0x0, 0x1, 0x0], [43]).into();

        let node = from_two_entries(first_entry, second_entry).unwrap();

        let mut branch_children = EMPTY_CHILDREN.clone();
        branch_children[0] = Some(Box::new(Node::leaf([0x1], [42])));
        branch_children[1] = Some(Box::new(Node::leaf([0x0], [43])));
        let expected_node_child = Node::Branch(branch_children, None);
        let expected_node = Node::extension([0x0], expected_node_child);

        assert_eq!(node, expected_node);
    }
}
