#[cfg(test)]
mod insert {
    use nybbles::Nibbles;

    use crate::node::{constructors::EMPTY_CHILDREN, Node};

    #[test]
    fn empty_key() {
        let node = Node::Null;
        assert_eq!(
            node.insert(Nibbles::unpack([]), [42]),
            Node::Branch(Default::default(), Some([42].into()))
        );
    }

    #[test]
    fn short_key() {
        let node = Node::Null;
        assert_eq!(
            node.insert(Nibbles::unpack([1]), [42]),
            Node::Leaf(Nibbles::unpack([1]).as_slice().into(), Box::new([42]))
        );
    }

    #[test]
    fn long_key() {
        let node = Node::Null;
        assert_eq!(
            node.insert(Nibbles::unpack([0x0, 0x0]), [42]),
            //here we create a leaf from nibbles, not from bytes as above
            Node::leaf([0x0, 0x0, 0x0, 0x0], [42])
        );
    }

    #[test]
    fn branch_with_two_children() {
        let node = Node::Null;
        let updated_node = node
            .insert(Nibbles::unpack([0x10]), [42])
            .insert(Nibbles::unpack([0x20]), [43]);

        let mut children = EMPTY_CHILDREN.clone();
        children[0x1] = Some(Box::new(Node::leaf([0x0], [42])));
        children[0x2] = Some(Box::new(Node::leaf([0x0], [43])));
        let expected_branch = Node::Branch(children, None);

        assert_eq!(expected_branch, updated_node);
    }
}
