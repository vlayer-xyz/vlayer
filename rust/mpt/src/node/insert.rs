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
                    *self = from_two_entries(
                        (**old_key).clone(),
                        old_value.clone(),
                        key,
                        value.as_ref().into(),
                    );
                }
            }
            _ => {}
        }
    }
}

fn split_first_nibble(nibbles: &Nibbles) -> (u8, Nibbles) {
    let splitted = nibbles.split_first().unwrap();
    let first_nibble = Nibbles::from_nibbles([*splitted.0]);
    let rest = Nibbles::from_nibbles(splitted.1);
    (first_nibble[0], rest)
}

fn from_two_entries(
    lhs_key: Nibbles,
    lhs_value: Box<[u8]>,
    rhs_key: Nibbles,
    rhs_value: Box<[u8]>,
) -> Node {
    if lhs_key.len() <= rhs_key.len() {
        from_two_ordered_entries(lhs_key, lhs_value, rhs_key, rhs_value)
    } else {
        from_two_ordered_entries(rhs_key, rhs_value, lhs_key, lhs_value)
    }
}

fn from_two_ordered_entries(
    lhs_key: Nibbles,
    lhs_value: Box<[u8]>,
    rhs_key: Nibbles,
    rhs_value: Box<[u8]>,
) -> Node {
    debug_assert!(lhs_key.len() <= rhs_key.len());
    if lhs_key == rhs_key {
        panic!("Key already exists")
    }
    let (rhs_first_nibble, remaining_rhs_key) = split_first_nibble(&rhs_key);
    dbg!(&rhs_first_nibble, &remaining_rhs_key);
    if lhs_key.is_empty() {
        let mut children: [Option<Box<Node>>; 16] = Default::default();
        children[rhs_first_nibble as usize] = Some(Box::new(Node::Leaf(
            KeyNibbles::from_nibbles(remaining_rhs_key),
            rhs_value,
        )));
        Node::Branch(children, Some(lhs_value))
    } else {
        let (lhs_first_nibble, remaining_lhs_key) = split_first_nibble(&lhs_key);
        dbg!(&lhs_first_nibble, &remaining_lhs_key);
        if lhs_first_nibble != rhs_first_nibble {
            let mut children: [Option<Box<Node>>; 16] = Default::default();
            children[lhs_first_nibble as usize] = Some(Box::new(Node::Leaf(
                KeyNibbles::from_nibbles(remaining_lhs_key),
                lhs_value,
            )));
            children[rhs_first_nibble as usize] = Some(Box::new(Node::Leaf(
                KeyNibbles::from_nibbles(remaining_rhs_key),
                rhs_value,
            )));
            Node::Branch(children, None)
        } else {
            todo!()
        }
    }
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
    #[should_panic(expected = "Key already exists")]
    fn twice_same_key() {
        let mut node = Node::Null;
        node.insert(Nibbles::unpack([0x1]), [42]);
        node.insert(Nibbles::unpack([0x1]), [43]);
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

    #[test]
    fn one_branch_one_leaf() {
        let mut node = Node::Null;
        node.insert(Nibbles::unpack([0x11]), [42]);
        node.insert(Nibbles::unpack([]), [43]);
        let mut expected_branch = Node::Branch(Default::default(), Some(Box::new([43])));

        if let Node::Branch(ref mut children, _) = expected_branch {
            children[0x1] = Some(Box::new(Node::leaf([0x1], [42])));
        }

        assert_eq!(expected_branch, node);
    }

    #[test]
    //todo - make it pass
    #[ignore]
    fn one_leaf_one_branch() {
        let mut node = Node::Null;
        node.insert(Nibbles::unpack([]), [42]);
        node.insert(Nibbles::unpack([0x11]), [43]);
        let mut expected_branch = Node::Branch(Default::default(), Some(Box::new([42])));

        if let Node::Branch(ref mut children, _) = expected_branch {
            children[0x1] = Some(Box::new(Node::leaf([0x1], [43])));
        }

        assert_eq!(expected_branch, node);
    }
}
