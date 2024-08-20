#[cfg(test)]
mod insert {
    use nybbles::Nibbles;

    use crate::node::Node;

    #[test]
    fn empty_key() {
        let node = Node::Null.insert(Nibbles::unpack([]), [42]);
        assert_eq!(Node::Branch(Default::default(), Some([42].into())), node);
    }

    #[test]
    fn short_key() {
        let node = Node::Null.insert(Nibbles::unpack([1]), [42]);
        assert_eq!(
            Node::Leaf(Nibbles::unpack([1]).as_slice().into(), Box::new([42])),
            node
        );
    }

    #[test]
    fn long_key() {
        let node = Node::Null.insert(Nibbles::unpack([0xf, 0xf, 0xf, 0xf]), [42]);
        assert_eq!(
            Node::Leaf(
                Nibbles::unpack([0xf, 0xf, 0xf, 0xf]).as_slice().into(),
                Box::new([42])
            ),
            node
        );
    }

    #[test]
    fn branch_with_two_children() {
        let node = Node::Null.insert(Nibbles::unpack([0x11]), [42]);
        let updated_node = node.insert(Nibbles::unpack([0x21]), [43]);
        let mut expected_branch = Node::Branch(Default::default(), None);

        if let Node::Branch(ref mut children, _) = expected_branch {
            children[0x1] = Some(Box::new(Node::create_leaf([0x1], [42])));
            children[0x2] = Some(Box::new(Node::create_leaf([0x1], [43])));
        }

        assert_eq!(expected_branch, updated_node);
    }
}
