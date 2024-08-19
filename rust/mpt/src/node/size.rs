use super::Node;

impl Node {
    /// Returns the number of full nodes in the trie.
    /// A full node is a node that needs to be fully encoded to compute the root hash.
    pub(crate) fn size(&self) -> usize {
        match self {
            Node::Null | Node::Digest(_) => 0,
            Node::Leaf(..) => 1,
            Node::Extension(_, child) => 1 + child.size(),
            Node::Branch(children, _) => {
                1 + children
                    .iter()
                    .filter_map(Option::as_deref)
                    .map(Node::size)
                    .sum::<usize>()
            }
        }
    }
}

#[cfg(test)]
mod node_size {
    use super::Node;
    use crate::key_nibbles::KeyNibbles;
    use std::array::from_fn;

    #[test]
    fn null() {
        let node = Node::Null;
        assert_eq!(node.size(), 0);
    }

    #[test]
    fn digest() {
        let node = Node::Digest(Default::default());
        assert_eq!(node.size(), 0);
    }

    #[test]
    fn leaf() {
        let node = Node::Leaf([0x1].into(), Box::new([]));
        assert_eq!(node.size(), 1);
    }

    #[test]
    fn extension() {
        let key_nibbles: KeyNibbles = [0x1].into();
        let leaf = Node::Leaf(key_nibbles.clone(), Box::new([]));
        let extension = Node::Extension(key_nibbles, Box::new(leaf));
        assert_eq!(extension.size(), 2);
    }

    #[test]
    fn branch_one_child() {
        let leaf = Node::Leaf([0x1].into(), Box::new([]));
        let child = Some(Box::new(leaf));
        const NULL_CHILD: Option<Box<Node>> = None;
        let mut children = [NULL_CHILD; 16];
        children[0] = child;
        let branch = Node::Branch(children, None);
        assert_eq!(branch.size(), 2);
    }

    #[test]
    fn branch_many_children() {
        let leaf = Node::Leaf([0x1].into(), Box::new([]));
        let child = Some(Box::new(leaf));
        let children: [_; 16] = from_fn(|_| child.clone());
        let branch = Node::Branch(children, None);
        assert_eq!(branch.size(), 17);
    }

    #[test]
    fn branch_with_value() {
        let leaf = Node::Leaf([0x1].into(), Box::new([]));
        let child = Some(Box::new(leaf));
        const NULL_CHILD: Option<Box<Node>> = None;
        let mut children = [NULL_CHILD; 16];
        children[0] = child;
        let value = Some([42u8].as_slice().into());
        let branch = Node::Branch(children, value);
        assert_eq!(branch.get(&[]), Some(&[42u8][..]));
    }
}
