use super::Node;

const EMPTY_CHILD: std::option::Option<Box<Node>> = None;
pub static EMPTY_CHILDREN: [Option<Box<Node>>; 16] = [EMPTY_CHILD; 16];
#[allow(unused)]
pub static EMPTY_BRANCH: Node = Node::Branch([EMPTY_CHILD; 16], None);

impl Node {
    #[allow(unused)]
    pub(crate) fn leaf(key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
        Node::Leaf(key_nibs.into(), value.as_ref().into())
    }

    pub(crate) fn extension(key_nibs: impl AsRef<[u8]>, value: Node) -> Node {
        Node::Extension(key_nibs.into(), value.into())
    }

    #[allow(unused)]
    pub(crate) fn branch_with_value(
        children: [Option<Box<Node>>; 16],
        value: impl AsRef<[u8]>,
    ) -> Node {
        Node::Branch(children, Some(value.as_ref().into()))
    }
}
