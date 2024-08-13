use core::panic;
use std::array::from_fn;

use super::Node;

fn leaf(key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
    Node::Leaf(key_nibs.into(), value.as_ref().into())
}

fn branch(value: impl AsRef<[u8]>) -> Node {
    let children = from_fn(|_| None);
    Node::Branch(children, Some(value.as_ref().into()))
}

impl Node {
    #[allow(unused)]
    pub fn insert(self, key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
        match self {
            Node::Null => {
                if key_nibs.as_ref().is_empty() {
                    branch(value)
                } else {
                    leaf(key_nibs, value)
                }
            }
            Node::Digest(_) => panic!("Cannot insert into a digest node"),
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
}
