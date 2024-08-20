use crate::key_nibbles::KeyNibbles;

use super::Node;
use nybbles::Nibbles;

impl Node {
    #[allow(unused)]
    pub(crate) fn insert(self, key: Nibbles, value: impl AsRef<[u8]>) -> Node {
        let nibble = key.clone().pop();
        match self {
            Node::Null => Node::insert_null(key, value),
            Node::Leaf(old_key, old_value) => {
                Node::insert_into_leaf(&old_key, &old_value, key, value)
            }
            _ => panic!("Not implemented"),
        }
    }

    fn insert_null(key: Nibbles, value: impl AsRef<[u8]>) -> Node {
        if key.is_empty() {
            Node::Branch(Default::default(), Some(value.as_ref().into()))
        } else {
            Node::create_leaf(key.as_slice(), value)
        }
    }

    fn insert_into_leaf(
        old_key: &Nibbles,
        old_value: &[u8],
        key: Nibbles,
        value: impl AsRef<[u8]>,
    ) -> Node {
        if **old_key == key {
            panic!("Key already exists");
        } else {
            from_two_entries(
                (*old_key).clone(),
                old_value.into(),
                key,
                value.as_ref().to_vec().into_boxed_slice(),
            )
        }
    }
}

pub fn split_first_nibble(nibbles: &Nibbles) -> Result<(u8, Nibbles), &'static str> {
    if let Some(splitted) = nibbles.split_first() {
        let first_nibble = Nibbles::from_nibbles([*splitted.0]);
        let rest = Nibbles::from_nibbles(splitted.1);
        Ok((first_nibble[0], rest))
    } else {
        Err("Nibbles is empty")
    }
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
    let (rhs_first_nibble, remaining_rhs_key) = split_first_nibble(&rhs_key).unwrap();
    dbg!(&rhs_first_nibble, &remaining_rhs_key);
    if lhs_key.is_empty() {
        let mut children: [Option<Box<Node>>; 16] = Default::default();
        children[rhs_first_nibble as usize] = Some(Box::new(Node::Leaf(
            KeyNibbles::from_nibbles(remaining_rhs_key),
            rhs_value,
        )));
        Node::Branch(children, Some(lhs_value))
    } else {
        let (lhs_first_nibble, remaining_lhs_key) = split_first_nibble(&lhs_key).unwrap();
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
            let node = from_two_ordered_entries(
                remaining_lhs_key,
                lhs_value,
                remaining_rhs_key,
                rhs_value,
            );
            match node {
                Node::Branch(_, _) => Node::Extension(
                    KeyNibbles::from_nibbles(Nibbles::from_nibbles([lhs_first_nibble])),
                    Box::new(node),
                ),
                Node::Extension(key, value) => Node::Extension(
                    KeyNibbles::from_nibbles(Nibbles::from_nibbles([lhs_first_nibble])),
                    Box::new(Node::Extension(key, value)),
                ),
                _ => todo!(),
            }
        }
    }
}
