use core::panic;
use std::array::from_fn;

use crate::node::insert::utils::branch;

use self::entry::Entry;

use super::Node;

pub mod entry;
mod utils;

// Expects lhs key length to be less or equal to rhs key length.
fn from_two_ordered_entries(lhs: Entry, rhs: Entry) -> Node {
    debug_assert!(lhs.key.len() <= rhs.key.len());
    if lhs.key == rhs.key {
        panic!("Key already exists")
    }
    if lhs.key.is_empty() {
        let mut children: [Option<Box<Node>>; 16] = from_fn(|_| None);
        let (rhs_nibble, rhs) = rhs.split_first_key_nibble();
        let node: Node = rhs.into();
        children[rhs_nibble as usize] = Some(Box::new(node));
        return branch(children, lhs.value);
    };

    unimplemented!()
}

fn from_two_entries(lhs: impl Into<Entry>, rhs: impl Into<Entry>) -> Node {
    let lhs: Entry = lhs.into();
    let rhs: Entry = rhs.into();
    if lhs.key.len() <= rhs.key.len() {
        from_two_ordered_entries(lhs, rhs)
    } else {
        from_two_ordered_entries(rhs, lhs)
    }
}

impl Node {
    #[allow(unused)]
    pub fn insert(self, key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
        match self {
            Node::Null => {
                let entry: Entry = (key_nibs, value).into();
                entry.into()
            }
            Node::Digest(_) => panic!("Cannot insert into a digest node"),
            Node::Leaf(old_key_nibs, old_value) => {
                let old_entry = (&**old_key_nibs, &*old_value);
                let entry = (key_nibs.as_ref(), value.as_ref());
                from_two_entries(old_entry, entry)
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod node_insert {
    use crate::node::Node;

    mod null {
        use super::*;

        #[test]
        fn non_empty_key() {
            let node = Node::Null;
            let updated_node = node.insert([1], [42]);
            assert_eq!(updated_node.get([1]).unwrap(), [42]);
        }

        #[test]
        fn empty_key() {
            let node = Node::Null;
            let updated_node = node.insert([], [42]);
            assert_eq!(updated_node.get([]).unwrap(), [42]);
        }
    }

    mod digest {
        use super::*;

        #[test]
        #[should_panic(expected = "Cannot insert into a digest node")]
        fn panics() {
            let node = Node::Digest(Default::default());
            node.insert([1], [42]);
        }
    }

    mod leaf {
        use super::*;
        use crate::node::insert::utils::leaf;
        use assert_matches::assert_matches;

        #[test]
        #[should_panic(expected = "Key already exists")]
        fn override_attempt() {
            let node = leaf([1], [42]);
            node.insert([1], [42]);
        }

        #[test]
        fn no_common_prefix() {
            let node = leaf([1], [42]);
            let updated_node = node.insert([2], [43]);

            assert_eq!(updated_node.get([1]).unwrap(), [42]);
            assert_eq!(updated_node.get([2]).unwrap(), [43]);
            assert_matches!(updated_node, Node::Branch(_, None));
        }

        #[test]
        fn common_prefix() {
            let node = leaf([1, 2], [42]);
            let updated_node = node.insert([1, 3], [43]);

            assert_eq!(updated_node.get([1, 2]).unwrap(), [42]);
            assert_eq!(updated_node.get([1, 3]).unwrap(), [43]);
            assert_matches!(updated_node, Node::Extension(_, _));
        }
    }
}
