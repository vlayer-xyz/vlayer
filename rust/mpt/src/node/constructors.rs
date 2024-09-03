use crate::key_nibbles::KeyNibbles;

use super::Node;

pub const EMPTY_CHILD: std::option::Option<Box<Node>> = None;
pub static EMPTY_CHILDREN: [Option<Box<Node>>; 16] = [EMPTY_CHILD; 16];

impl Node {
    #[allow(unused)]
    pub(crate) fn leaf(key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
        Node::Leaf(key_nibs.into(), value.as_ref().into())
    }

    pub(crate) fn extension(key_nibs: impl AsRef<[u8]>, value: Node) -> Node {
        Node::Extension(key_nibs.into(), value.into())
    }

    #[allow(unused)]
    pub(crate) fn branch_with_child(idx: u8, child: impl Into<Node>) -> Node {
        let mut children = EMPTY_CHILDREN.clone();
        children[idx as usize] = Some(Box::new(child.into()));
        Node::Branch(children, None)
    }

    #[allow(unused)]
    pub(crate) fn branch_with_value(value: impl AsRef<[u8]>) -> Node {
        Node::branch_with_children_and_value(EMPTY_CHILDREN.clone(), value)
    }

    pub(crate) fn branch_with_child_and_value(
        idx: u8,
        child: impl Into<Node>,
        value: impl AsRef<[u8]>,
    ) -> Node {
        let mut children = EMPTY_CHILDREN.clone();
        children[idx as usize] = Some(Box::new(child.into()));
        Node::branch_with_children_and_value(children, value)
    }

    #[allow(unused)]
    pub(crate) fn branch_with_two_children(
        first_idx: u8,
        first_child: impl Into<Node>,
        second_idx: u8,
        second_child: impl Into<Node>,
    ) -> Node {
        let mut children = EMPTY_CHILDREN.clone();
        children[first_idx as usize] = Some(Box::new(first_child.into()));
        children[second_idx as usize] = Some(Box::new(second_child.into()));
        Node::Branch(children, None)
    }

    #[allow(unused)]
    pub(crate) fn branch_with_child_node(key: KeyNibbles, child_node: impl Into<Node>) -> Node {
        let (first_key_nibble, remaining_key_nibbles) = key.split_first();
        let node: Node = if remaining_key_nibbles.as_ref().is_empty() {
            child_node.into()
        } else {
            Node::extension(remaining_key_nibbles, child_node.into())
        };

        Node::branch_with_child(first_key_nibble, node)
    }

    fn branch_with_children_and_value(
        children: [Option<Box<Node>>; 16],
        value: impl AsRef<[u8]>,
    ) -> Node {
        Node::Branch(children, Some(value.as_ref().into()))
    }
}
