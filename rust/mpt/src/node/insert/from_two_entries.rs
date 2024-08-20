use nybbles::Nibbles;

use crate::{
    key_nibbles::KeyNibbles,
    node::{insert::split_first_nibble, Node},
};

impl Node {
    pub fn from_two_entries(
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
        create_branch_with_leaf(rhs_first_nibble, remaining_rhs_key, rhs_value, lhs_value)
    } else {
        let (lhs_first_nibble, remaining_lhs_key) = split_first_nibble(&lhs_key).unwrap();
        if lhs_first_nibble != rhs_first_nibble {
            create_branch_with_two_leaves(
                lhs_first_nibble,
                remaining_lhs_key,
                lhs_value,
                rhs_first_nibble,
                remaining_rhs_key,
                rhs_value,
            )
        } else {
            extend_with_branch_or_extension(
                lhs_first_nibble,
                remaining_lhs_key,
                lhs_value,
                remaining_rhs_key,
                rhs_value,
            )
        }
    }
}

fn create_branch_with_leaf(
    rhs_first_nibble: u8,
    remaining_rhs_key: Nibbles,
    rhs_value: Box<[u8]>,
    lhs_value: Box<[u8]>,
) -> Node {
    let mut children: [Option<Box<Node>>; 16] = Default::default();
    children[rhs_first_nibble as usize] = Some(Box::new(Node::Leaf(
        KeyNibbles::from_nibbles(remaining_rhs_key),
        rhs_value,
    )));
    Node::Branch(children, Some(lhs_value))
}

fn create_branch_with_two_leaves(
    lhs_first_nibble: u8,
    remaining_lhs_key: Nibbles,
    lhs_value: Box<[u8]>,
    rhs_first_nibble: u8,
    remaining_rhs_key: Nibbles,
    rhs_value: Box<[u8]>,
) -> Node {
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
}

fn extend_with_branch_or_extension(
    lhs_first_nibble: u8,
    remaining_lhs_key: Nibbles,
    lhs_value: Box<[u8]>,
    remaining_rhs_key: Nibbles,
    rhs_value: Box<[u8]>,
) -> Node {
    let node = from_two_ordered_entries(remaining_lhs_key, lhs_value, remaining_rhs_key, rhs_value);
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
