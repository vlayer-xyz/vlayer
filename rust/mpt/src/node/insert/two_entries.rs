use std::array::from_fn;

use crate::node::{insert::utils::branch, Node};

use super::entry::Entry;

// Expects lhs key length to be less or equal to rhs key length.
fn from_two_ordered_entries(lhs: Entry, rhs: Entry) -> Node {
    debug_assert!(lhs.key.len() <= rhs.key.len());
    if lhs.key == rhs.key {
        panic!("Key already exists")
    }
    let (rhs_nibble, rhs) = rhs.split_first_key_nibble();
    if lhs.key.is_empty() {
        let mut children: [Option<Box<Node>>; 16] = from_fn(|_| None);
        children[rhs_nibble as usize] = Some(Box::new(rhs.into()));

        return branch(children, Some(lhs.value));
    } else {
        let (lhs_nibble, lhs) = lhs.split_first_key_nibble();
        if lhs_nibble != rhs_nibble {
            let mut children: [Option<Box<Node>>; 16] = from_fn(|_| None);
            children[lhs_nibble as usize] = Some(Box::new(lhs.into()));
            children[rhs_nibble as usize] = Some(Box::new(rhs.into()));

            return branch(children, None);
        } else {
            let node = from_two_ordered_entries(lhs, rhs);
            match node {
                Node::Branch(_, _) => Node::Extension([lhs_nibble].into(), Box::new(node)),
                Node::Extension(nibbles, child) => {
                    Node::Extension(nibbles.push_front(lhs_nibble), child)
                }
                _ => unreachable!("Unexpected node type"),
            }
        }
    }
}

pub fn from_two_entries(lhs: impl Into<Entry>, rhs: impl Into<Entry>) -> Node {
    let lhs: Entry = lhs.into();
    let rhs: Entry = rhs.into();
    if lhs.key.len() <= rhs.key.len() {
        from_two_ordered_entries(lhs, rhs)
    } else {
        from_two_ordered_entries(rhs, lhs)
    }
}
