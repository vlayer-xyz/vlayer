use self::{entry::Entry, two_entries::from_two_entries};
use core::panic;

use super::Node;

pub mod entry;
mod two_entries;
mod utils;

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
