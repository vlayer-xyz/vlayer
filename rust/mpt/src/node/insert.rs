use crate::key_nibbles::KeyNibbles;

use super::Node;
use nybbles::Nibbles;

impl Node {
    #[allow(unused)]
    pub(crate) fn insert(&mut self, key: Nibbles, value: impl AsRef<[u8]>) {
        let nibble = key.clone().pop();
        match self {
            Node::Null => {
                if key.is_empty() {
                    *self = Node::Branch(Default::default(), Some(value.as_ref().into()));
                } else {
                    *self = Node::leaf(key.as_slice(), value);
                }
            }
            Node::Leaf(old_key, old_value) => {
                if **old_key == key {
                    panic!("Key already exists");
                } else {
                    let (old_key_first_nibble, remaining_old_key) = split_first_nibble(old_key);
                    let (key_first_nibble, remaining_key) = split_first_nibble(&key);

                    let mut children: [Option<Box<Node>>; 16] = Default::default();
                    children[old_key_first_nibble as usize] = Some(Box::new(Node::Leaf(
                        KeyNibbles::from_nibbles(remaining_old_key),
                        old_value.clone(),
                    )));
                    children[key_first_nibble as usize] = Some(Box::new(Node::Leaf(
                        KeyNibbles::from_nibbles(remaining_key),
                        value.as_ref().into(),
                    )));

                    *self = Node::Branch(children, None);
                }
            }
            _ => {}
        }
    }
}

fn split_first_nibble(nibbles: &Nibbles) -> (u8, Nibbles) {
    dbg!(nibbles);
    let splitted = nibbles.split_first().unwrap();
    dbg!(splitted);
    let first_nibble = Nibbles::from_nibbles([*splitted.0]);
    let rest = Nibbles::from_nibbles(splitted.1);
    dbg!(&first_nibble, &rest);
    (first_nibble[0], rest)
}

#[cfg(test)]
mod insert {
    use super::*;

    #[test]
    fn empty_key() {
        let mut node = Node::Null;
        node.insert(Nibbles::unpack([]), [42]);
        assert_eq!(Node::Branch(Default::default(), Some([42].into())), node);
    }

    #[test]
    fn short_key() {
        let mut node = Node::Null;
        node.insert(Nibbles::unpack([1]), [42]);
        assert_eq!(
            Node::Leaf(Nibbles::unpack([1]).as_slice().into(), Box::new([42])),
            node
        );
    }

    #[test]
    fn long_key() {
        let mut node = Node::Null;
        node.insert(Nibbles::unpack([0xf, 0xf, 0xf, 0xf]), [42]);
        assert_eq!(
            Node::Leaf(
                Nibbles::unpack([0xf, 0xf, 0xf, 0xf]).as_slice().into(),
                Box::new([42])
            ),
            node
        );
    }

    #[test]
    fn branch_with_two_children() {
        let mut node = Node::Null;
        node.insert(Nibbles::unpack([0x11]), [42]);
        node.insert(Nibbles::unpack([0x21]), [43]);
        let mut expected_branch = Node::Branch(Default::default(), None);

        if let Node::Branch(ref mut children, _) = expected_branch {
            children[0x1] = Some(Box::new(Node::leaf([0x1], [42])));
            children[0x2] = Some(Box::new(Node::leaf([0x1], [43])));
        }

        assert_eq!(expected_branch, node);
    }
}
