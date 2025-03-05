// from_two_entries is a helper function used in Node::insert to handle inserting into Leaf node. It is used to reduce
// the number of cases to handle - we convert Leaf's key and value into an Entry struct and then use from_two_entries which
// treats both new entry and Leaf's entry symmetrically.

use nybbles::Nibbles;

use super::entry::Entry;
use crate::node::{Node, NodeError};

pub(crate) fn from_two_entries<D>(
    lhs: impl Into<Entry>,
    rhs: impl Into<Entry>,
) -> Result<Node<D>, NodeError> {
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

fn from_two_ordered_entries<D>(shorter: Entry, longer: Entry) -> Result<Node<D>, NodeError> {
    if shorter.key == longer.key {
        return Err(NodeError::DuplicateKey);
    }

    // We know longer.key can't be empty, since the case of equal keys was handled above.
    // ![Shorter key empty](../../../images/into_leaf_0.png)
    if shorter.key.is_empty() {
        return Ok(from_value_and_entry(shorter.value, longer));
    }

    let (shorter_first_nibble, remaining_shorter) = shorter.split_first_key_nibble();
    let (longer_first_nibble, remaining_longer) = longer.split_first_key_nibble();

    // ![Different first nibbles](../../../images/into_leaf_1.png)
    if shorter_first_nibble != longer_first_nibble {
        return Ok(Node::branch_with_two_children(
            shorter_first_nibble,
            remaining_shorter,
            longer_first_nibble,
            remaining_longer,
        ));
    }

    // ![Common prefix nonempty](../../../images/into_leaf_2.png)
    let node = from_two_entries(remaining_shorter, remaining_longer)?;
    let result_node = prepend_nibble(shorter_first_nibble, node);

    Ok(result_node)
}

fn from_value_and_entry<D>(value: impl AsRef<[u8]>, entry: Entry) -> Node<D> {
    let (nibble, remaining) = entry.split_first_key_nibble();
    Node::branch_with_child_and_value(nibble, remaining, value)
}

fn prepend_nibble<D>(nibble: u8, node: Node<D>) -> Node<D> {
    match node {
        Node::Branch(_, _) => Node::extension([nibble], node),
        Node::Extension(nibbles, child) => {
            let nibble = Nibbles::from_nibbles([nibble]);
            Node::Extension(nibble.join(&nibbles), child)
        }
        _ => unreachable!("from_two_ordered_entries should return only Branch or Extension"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Keccak256;

    type D = Keccak256;

    #[test]
    fn two_empty_keys() {
        let first_entry = ([], [42]);
        let second_entry = ([], [43]);

        let result = from_two_entries::<D>(first_entry, second_entry);
        assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
    }

    #[test]
    fn one_empty_key() -> anyhow::Result<()> {
        let first_entry = ([], [42]);
        let second_entry = ([0x0], [43]);
        let node = from_two_entries::<D>(first_entry, second_entry)?;

        let expected_node = Node::branch_with_child_and_value(0, Node::leaf([], [43]), [42]);

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn duplicate_key() {
        let old_entry = ([0], [42]);
        let entry = ([0], [43]);
        let result = from_two_entries::<D>(old_entry, entry);
        assert_eq!(result.unwrap_err(), NodeError::DuplicateKey);
    }

    #[test]
    fn different_single_nibbles() -> anyhow::Result<()> {
        let first_entry = ([0x0], [42]);
        let second_entry = ([0x1], [43]);

        let node = from_two_entries::<D>(first_entry, second_entry)?;

        let expected_node =
            Node::branch_with_two_children(0, Node::leaf([], [42]), 1, Node::leaf([], [43]));

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn no_common_prefix() -> anyhow::Result<()> {
        let first_entry = ([0x0, 0x0], [42]);
        let second_entry = ([0x1, 0x0], [43]);
        let node = from_two_entries::<D>(first_entry, second_entry)?;

        let expected_node =
            Node::branch_with_two_children(0, Node::leaf([0], [42]), 1, Node::leaf([0], [43]));

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn common_prefix() -> anyhow::Result<()> {
        let first_entry = ([0x0, 0x0], [42]);
        let second_entry = ([0x0, 0x1], [43]);

        let node = from_two_entries::<D>(first_entry, second_entry)?;

        let expected_child_node =
            Node::branch_with_two_children(0, Node::leaf([], [42]), 1, Node::leaf([], [43]));
        let expected_node = Node::extension([0x0], expected_child_node);

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn long_common_prefix() -> anyhow::Result<()> {
        let first_entry = ([0x0, 0x1, 0x0], [42]);
        let second_entry = ([0x0, 0x1, 0x1], [43]);

        let node = from_two_entries::<D>(first_entry, second_entry)?;

        let expected_child_node =
            Node::branch_with_two_children(0, Node::leaf([], [42]), 1, Node::leaf([], [43]));
        let expected_node = Node::extension([0x0, 0x1], expected_child_node);

        assert_eq!(node, expected_node);
        Ok(())
    }

    #[test]
    fn common_prefix_with_different_long_suffix() -> anyhow::Result<()> {
        let first_entry = ([0x0, 0x0, 0x1], [42]);
        let second_entry = ([0x0, 0x1, 0x0], [43]);

        let node = from_two_entries::<D>(first_entry, second_entry)?;

        let expected_child_node =
            Node::branch_with_two_children(0, Node::leaf([0x1], [42]), 1, Node::leaf([0x0], [43]));

        let expected_node = Node::extension([0x0], expected_child_node);

        assert_eq!(node, expected_node);
        Ok(())
    }
}
