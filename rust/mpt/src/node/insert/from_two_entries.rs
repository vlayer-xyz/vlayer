use crate::node::{constructors::EMPTY_CHILDREN, Node};

use super::entry::Entry;

pub fn from_two_entries(lhs: Entry, rhs: Entry) -> Node {
    if lhs.key == rhs.key {
        panic!("Key already exists");
    } else {
        if lhs.key.is_empty() || rhs.key.is_empty() {
            todo!("Handle empty key case");
        }
        let (lhs_first_nibble, remaining_lhs) = lhs.split_first_key_nibble();
        let (rhs_first_nibble, remaining_rhs) = rhs.split_first_key_nibble();

        if lhs_first_nibble != rhs_first_nibble {
            let mut children = EMPTY_CHILDREN.clone();
            children[lhs_first_nibble as usize] = Some(Box::new(remaining_lhs.into()));
            children[rhs_first_nibble as usize] = Some(Box::new(remaining_rhs.into()));

            Node::Branch(children, None)
        } else {
            todo!("Extend with branch or extension");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Key already exists")]
    fn duplicate_key() {
        let old_entry: Entry = ([0], [42]).into();
        let entry: Entry = ([0], [43]).into();
        from_two_entries(old_entry, entry);
    }

    #[test]
    fn two_nibbles() {
        let first_entry: Entry = ([0x0], [42]).into();
        let second_entry: Entry = ([0x1], [43]).into();
        let node = from_two_entries(first_entry, second_entry);

        if let Node::Branch(children, _) = node {
            assert_eq!(
                children[0],
                Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([42]))))
            );
            assert_eq!(
                children[1],
                Some(Box::new(Node::branch(EMPTY_CHILDREN.clone(), Some([43]))))
            );
        }
    }

    #[test]
    fn two_long_nibbles() {
        let first_entry: Entry = ([0x0, 0x0], [42]).into();
        let second_entry: Entry = ([0x1, 0x0], [43]).into();
        let node = from_two_entries(first_entry, second_entry);

        if let Node::Branch(children, _) = node {
            assert_eq!(children[0], Some(Box::new(Node::leaf([0], [42]))));
            assert_eq!(children[1], Some(Box::new(Node::leaf([0], [43]))));
        }
    }
}
