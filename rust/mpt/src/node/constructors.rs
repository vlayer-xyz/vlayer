use super::Node;

const EMPTY_CHILD: std::option::Option<Box<Node>> = None;
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
    pub(crate) fn branch(
        children: [Option<Box<Node>>; 16],
        value: Option<impl AsRef<[u8]>>,
    ) -> Node {
        let value = value.map(|v| v.as_ref().into());
        Node::Branch(children, value)
    }
}
