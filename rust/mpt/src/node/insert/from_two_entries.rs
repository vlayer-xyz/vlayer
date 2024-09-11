use crate::node::{Node, NodeError};

use super::entry::Entry;

fn order_entries(lhs: Entry, rhs: Entry) -> (Entry, Entry) {
    if lhs.key.len() <= rhs.key.len() {
        (lhs, rhs)
    } else {
        (rhs, lhs)
    }
}

pub(crate) fn from_two_entries(
    lhs: impl Into<Entry>,
    rhs: impl Into<Entry>,
) -> Result<Node, NodeError> {
    let lhs = lhs.into();
    let rhs = rhs.into();

    if lhs.key == rhs.key {
        return Err(NodeError::DuplicateKey);
    }
    let (shorter, longer) = order_entries(lhs, rhs);

    if shorter.key.is_empty() {
        let (longer_first_nibble, remaining_longer) = longer.split_first_key_nibble();
        return Ok(Node::branch_with_child_and_value(
            longer_first_nibble,
            remaining_longer,
            shorter.value,
        ));
    }
    let (shorter_first_nibble, remaining_shorter) = shorter.split_first_key_nibble();
    let (longer_first_nibble, remaining_longer) = longer.split_first_key_nibble();

    if shorter_first_nibble != longer_first_nibble {
        return Ok(Node::branch_with_two_children(
            shorter_first_nibble,
            remaining_shorter,
            longer_first_nibble,
            remaining_longer,
        ));
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
    fn two_empty_keys() {
        let first_entry = ([], [42]);
        let second_entry = ([], [43]);

        let result = from_two_entries(first_entry, second_entry);
        assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
    }

    #[test]
    fn one_empty_key() -> anyhow::Result<()> {
        let first_entry = ([], [42]);
        let second_entry = ([0x0], [43]);
        let node = from_two_entries(first_entry, second_entry)?;

        let expected_node =
            Node::branch_with_child_and_value(0, Node::branch_with_value([43]), [42]);

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn duplicate_key() {
        let old_entry = ([0], [42]);
        let entry = ([0], [43]);
        let result = from_two_entries(old_entry, entry);
        assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
    }

    #[test]
    fn different_single_nibbles() -> anyhow::Result<()> {
        let first_entry = ([0x0], [42]);
        let second_entry = ([0x1], [43]);

        let node = from_two_entries(first_entry, second_entry)?;

        let expected_node = Node::branch_with_two_children(
            0,
            Node::branch_with_value([42]),
            1,
            Node::branch_with_value([43]),
        );

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn no_common_prefix() -> anyhow::Result<()> {
        let first_entry = ([0x0, 0x0], [42]);
        let second_entry = ([0x1, 0x0], [43]);
        let node = from_two_entries(first_entry, second_entry)?;

        let expected_node =
            Node::branch_with_two_children(0, Node::leaf([0], [42]), 1, Node::leaf([0], [43]));

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn common_prefix() -> anyhow::Result<()> {
        let first_entry = ([0x0, 0x0], [42]);
        let second_entry = ([0x0, 0x1], [43]);

        let node = from_two_entries(first_entry, second_entry)?;

        let expected_child_node = Node::branch_with_two_children(
            0,
            Node::branch_with_value([42]),
            1,
            Node::branch_with_value([43]),
        );
        let expected_node = Node::extension([0x0], expected_child_node);

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn long_common_prefix() -> anyhow::Result<()> {
        let first_entry = ([0x0, 0x1, 0x0], [42]);
        let second_entry = ([0x0, 0x1, 0x1], [43]);

        let node = from_two_entries(first_entry, second_entry)?;

        let expected_child_node = Node::branch_with_two_children(
            0,
            Node::branch_with_value([42]),
            1,
            Node::branch_with_value([43]),
        );
        let expected_node = Node::extension([0x0, 0x1], expected_child_node);

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn common_prefix_with_different_long_suffix() -> anyhow::Result<()> {
        let first_entry = ([0x0, 0x0, 0x1], [42]);
        let second_entry = ([0x0, 0x1, 0x0], [43]);

        let node = from_two_entries(first_entry, second_entry)?;

        let expected_child_node =
            Node::branch_with_two_children(0, Node::leaf([0x1], [42]), 1, Node::leaf([0x0], [43]));

        let expected_node = Node::extension([0x0], expected_child_node);

        assert_eq!(node, expected_node);
        Ok(())
    }
}
