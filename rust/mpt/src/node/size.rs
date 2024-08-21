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
    use crate::node::Node;
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
        let node = Node::leaf([0x1], []);
        assert_eq!(node.size(), 1);
    }

    #[test]
    fn extension() {
        let leaf = Node::leaf([0x1], []);
        let extension = Node::extension([0x1], leaf);
        assert_eq!(extension.size(), 2);
    }

    #[test]
    fn branch_one_child() {
        let leaf = Node::leaf([0x1], []);
        let mut children: [Option<Box<Node>>; 16] = Default::default();
        children[0] = Some(Box::new(leaf));
        let branch = Node::Branch(children, None);

        assert_eq!(branch.size(), 2);
    }

    #[test]
    fn branch_many_children() {
        let leaf = Node::leaf([0x1], []);
        let child = Some(Box::new(leaf));
        let children: [_; 16] = from_fn(|_| child.clone());
        let branch = Node::Branch(children, None);

        assert_eq!(branch.size(), 17);
    }

    #[test]
    fn branch_with_value() {
        let leaf = Node::leaf([0x1], []);
        let mut children: [Option<Box<Node>>; 16] = Default::default();
        children[0] = Some(Box::new(leaf));
        let branch = Node::branch(children, Some([42]));

        assert_eq!(branch.get([]).unwrap(), [42]);
    }
}
