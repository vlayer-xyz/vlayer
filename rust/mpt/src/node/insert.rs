use super::Node;

fn leaf(key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
    Node::Leaf(key_nibs.into(), value.as_ref().into())
}

impl Node {
    #[allow(unused)]
    pub fn insert(self, key_nibs: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Node {
        match self {
            Node::Null => leaf(key_nibs, value),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod node_insert {
    use crate::node::Node;

    mod null {
        use super::*;

        #[test]
        fn non_empty_key() {
            let node = Node::Null;
            let updated_node = node.insert([1], [42]);
            assert_eq!(updated_node.get([1]).unwrap(), [42]);
        }
    }
}
