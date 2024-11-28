use super::Node;

impl<D> Node<D> {
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
            Node::_Phantom(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod node_size {
    use std::array::from_fn;

    use crate::node::KeccakNode as Node;

    #[test]
    fn null() {
        let node = Node::null();
        assert_eq!(node.size(), 0);
    }

    #[test]
    fn digest() {
        let node = Node::digest(Default::default());
        assert_eq!(node.size(), 0);
    }

    #[test]
    fn leaf() {
        let node = Node::leaf([0x0], [0]);
        assert_eq!(node.size(), 1);
    }

    #[test]
    fn extension() {
        let leaf = Node::leaf([0x0], [0]);
        let extension = Node::extension([0x0], leaf);
        assert_eq!(extension.size(), 2);
    }

    #[test]
    fn branch_one_child() {
        let leaf = Node::leaf([0x0], [0]);
        let branch = Node::branch_with_child(0, leaf);

        assert_eq!(branch.size(), 2);
    }

    #[test]
    fn branch_many_children() {
        let leaf = Node::leaf([0x0], [0]);
        let child = Some(Box::new(leaf));
        let children = from_fn(|_| child.clone());
        let branch = Node::Branch(children, None);

        assert_eq!(branch.size(), 17);
    }

    #[test]
    fn branch_with_value() {
        let branch = Node::branch_with_value([42]);
        assert_eq!(branch.get([]).unwrap(), [42]);
    }
}
