use alloy_primitives::{B256, Bytes};
use nybbles::Nibbles;

use super::Node;

pub const fn empty_children<D>() -> [Option<Box<Node<D>>>; 16] {
    [const { None }; 16]
}

impl<D> Node<D> {
    #[allow(unused)]
    pub(crate) const fn null() -> Node<D> {
        Node::Null
    }

    #[allow(unused)]
    pub(crate) const fn digest(digest: B256) -> Node<D> {
        Node::Digest(digest)
    }

    pub(crate) fn leaf(key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node<D> {
        assert!(!value.as_ref().is_empty(), "empty values are not allowed in MPT");
        let key_nibs = Nibbles::from_nibbles(key_nibs.as_ref());
        Node::Leaf(key_nibs, Bytes::copy_from_slice(value.as_ref()))
    }

    pub(crate) fn extension(key_nibs: impl AsRef<[u8]>, value: impl Into<Node<D>>) -> Node<D> {
        let key_nibs = Nibbles::from_nibbles(key_nibs.as_ref());
        Node::Extension(key_nibs, value.into().into())
    }

    #[allow(unused)]
    pub(crate) fn branch_with_child(idx: u8, child: impl Into<Node<D>>) -> Node<D> {
        let mut children = empty_children();
        children[idx as usize] = Some(Box::new(child.into()));
        Node::Branch(children, None)
    }

    #[allow(unused)]
    pub(crate) fn branch_with_value(value: impl AsRef<[u8]>) -> Node<D> {
        assert!(!value.as_ref().is_empty(), "empty values are not allowed in MPT");
        Node::branch_with_children_and_value(empty_children(), value)
    }

    pub(crate) fn branch_with_child_and_value(
        idx: u8,
        child: impl Into<Node<D>>,
        value: impl AsRef<[u8]>,
    ) -> Node<D> {
        assert!(!value.as_ref().is_empty(), "empty values are not allowed in MPT");
        let mut children = empty_children();
        children[idx as usize] = Some(Box::new(child.into()));
        Node::branch_with_children_and_value(children, value)
    }

    pub(crate) fn branch_with_two_children(
        first_idx: u8,
        first_child: impl Into<Node<D>>,
        second_idx: u8,
        second_child: impl Into<Node<D>>,
    ) -> Node<D> {
        let mut children = empty_children();
        children[first_idx as usize] = Some(Box::new(first_child.into()));
        children[second_idx as usize] = Some(Box::new(second_child.into()));
        Node::Branch(children, None)
    }

    // `child_key` passed cannot be empty
    #[allow(clippy::expect_used)]
    pub(crate) fn branch_with_child_node(
        child_key: &Nibbles,
        child_node: impl Into<Node<D>>,
    ) -> Node<D> {
        let (first_key_nibble, remaining_key_nibbles) = child_key
            .split_first()
            .expect("child_key should not be empty");
        let node = if remaining_key_nibbles.is_empty() {
            child_node.into()
        } else {
            Node::extension(remaining_key_nibbles, child_node)
        };

        Node::branch_with_child(*first_key_nibble, node)
    }

    pub const fn branch_with_children(children: [Option<Box<Node<D>>>; 16]) -> Node<D> {
        Node::Branch(children, None)
    }

    pub const fn empty_branch() -> Node<D> {
        Node::Branch(empty_children(), None)
    }

    pub fn branch_with_children_and_value(
        children: [Option<Box<Node<D>>>; 16],
        value: impl AsRef<[u8]>,
    ) -> Node<D> {
        assert!(!value.as_ref().is_empty(), "empty values are not allowed in MPT");
        Node::Branch(children, Some(Bytes::copy_from_slice(value.as_ref())))
    }
}
