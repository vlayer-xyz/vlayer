use super::Node;
use nybbles::Nibbles;

mod from_two_entries;

impl Node {
    #[allow(unused)]
    pub(crate) fn insert(self, key: Nibbles, value: impl AsRef<[u8]>) -> Node {
        let nibble = key.clone().pop();
        match self {
            Node::Null => Node::insert_null(key, value),
            Node::Leaf(old_key, old_value) => {
                Node::insert_into_leaf(&old_key, &old_value, key, value)
            }
            _ => panic!("Not implemented"),
        }
    }

    fn insert_null(key: Nibbles, value: impl AsRef<[u8]>) -> Node {
        if key.is_empty() {
            Node::Branch(Default::default(), Some(value.as_ref().into()))
        } else {
            Node::create_leaf(key.as_slice(), value)
        }
    }

    fn insert_into_leaf(
        old_key: &Nibbles,
        old_value: &[u8],
        key: Nibbles,
        value: impl AsRef<[u8]>,
    ) -> Node {
        if **old_key == key {
            panic!("Key already exists");
        } else {
            Node::from_two_entries(
                (*old_key).clone(),
                old_value.into(),
                key,
                value.as_ref().to_vec().into_boxed_slice(),
            )
        }
    }
}

pub fn split_first_nibble(nibbles: &Nibbles) -> Result<(u8, Nibbles), &'static str> {
    if let Some(splitted) = nibbles.split_first() {
        let first_nibble = Nibbles::from_nibbles([*splitted.0]);
        let rest = Nibbles::from_nibbles(splitted.1);
        Ok((first_nibble[0], rest))
    } else {
        Err("Nibbles is empty")
    }
}
