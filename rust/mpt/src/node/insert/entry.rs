use std::array::from_fn;

use crate::node::Node;

use super::utils::leaf;

pub struct Entry {
    pub key: Box<[u8]>,
    pub value: Box<[u8]>,
}

impl<K, V> From<(K, V)> for Entry
where
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
{
    fn from((key, value): (K, V)) -> Self {
        Entry {
            key: key.as_ref().into(),
            value: value.as_ref().into(),
        }
    }
}

impl From<Entry> for Node {
    fn from(entry: Entry) -> Self {
        if entry.key.is_empty() {
            let children = from_fn(|_| None);
            Node::Branch(children, Some(entry.value))
        } else {
            leaf(entry.key, entry.value)
        }
    }
}

impl Entry {
    pub fn split_first_key_nibble(self) -> (u8, Entry) {
        let (first_nibble, rest) = self.key.split_first().unwrap();
        (
            *first_nibble,
            Entry {
                key: rest.into(),
                value: self.value,
            },
        )
    }
}
