use crate::node::Node;

pub fn leaf(key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
    Node::Leaf(key_nibs.into(), value.as_ref().into())
}

pub fn branch(children: [Option<Box<Node>>; 16], value: Option<Box<[u8]>>) -> Node {
    Node::Branch(children, value)
}
