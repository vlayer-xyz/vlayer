// from_two_entries is a helper function used in Node::insert to handle inserting into Leaf node. It is used to reduce
// the number of cases to handle - we convert Leaf's key and value into an Entry struct and then use from_two_entries which
// treats both new entry and Leaf's entry symmetrically.

use crate::node::{Node, NodeError};

use super::entry::Entry;

pub(crate) fn from_two_entries(
    lhs: impl Into<Entry>,
    rhs: impl Into<Entry>,
) -> Result<Node, NodeError> {
    let lhs = lhs.into();
    let rhs = rhs.into();

    let (shorter, longer) = order_entries(lhs, rhs);
    from_two_ordered_entries(shorter, longer)
}

fn order_entries(lhs: Entry, rhs: Entry) -> (Entry, Entry) {
    if lhs.key.len() <= rhs.key.len() {
        (lhs, rhs)
    } else {
        (rhs, lhs)
    }
}

fn from_two_ordered_entries(shorter: Entry, longer: Entry) -> Result<Node, NodeError> {
    if shorter.key == longer.key {
        return Err(NodeError::DuplicateKey);
    }

    // If the shorter key is empty, we create a Branch with a child and a value. Notice that we know
    // longer.key` can't be empty, since the case of equal keys was already handled above.
    // ![Schema](../../../images/into_leaf_0.png)
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

    // If both first nibbles exist and are different, we create a Branch with two children.
    // ![Schema](../../../images/into_leaf_1.png)
    if shorter_first_nibble != longer_first_nibble {
        return Ok(Node::branch_with_two_children(
            shorter_first_nibble,
            remaining_shorter,
            longer_first_nibble,
            remaining_longer,
        ));
    }

    // Here we extract recursively longest common prefix and then return Extension node with longest common prefix
    // as a key with a Branch as a child. This Branch has two children, each corresponding to one of the entries.
    // ![Schema](../../../images/into_leaf_2.png)
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
