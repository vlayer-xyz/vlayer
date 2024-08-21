use std::array::from_fn;

use nybbles::Nibbles;

use crate::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub key: Nibbles,
    pub value: Box<[u8]>,
}

impl<K, V> From<(K, V)> for Entry
where
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
{
    fn from((key, value): (K, V)) -> Self {
        Entry {
            key: Nibbles::from_nibbles(key.as_ref()),
            value: value.as_ref().into(),
        }
    }
}

impl From<Entry> for Node {
    fn from(Entry { key, value }: Entry) -> Self {
        if key.is_empty() {
            let children = from_fn(|_| None);
            Node::Branch(children, Some(value))
        } else {
            Node::leaf(&*key, value)
        }
    }
}

impl Entry {
    pub fn split_first_key_nibble(self) -> Result<(u8, Entry), &'static str> {
        if let Some((first_key_nibble, remaining_key)) = self.key.split_first() {
            let entry = (&*remaining_key, self.value).into();
            Ok((*first_key_nibble, entry))
        } else {
            Err("Nibbles is empty")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_nibbles() {
        let entry: Entry = ([], []).into();
        assert_eq!(entry.split_first_key_nibble(), Err("Nibbles is empty"));
    }

    #[test]
    fn single_nibble() {
        let entry: Entry = ([0x0], []).into();

        let (first, rest_entry) = entry.split_first_key_nibble().unwrap();

        assert_eq!(first, 0x0);
        assert_eq!(*rest_entry.value, []);
    }

    #[test]
    fn non_empty_nibbles() {
        let entry: Entry = ([0x0, 0x1], []).into();

        let (first, rest_entry) = entry.split_first_key_nibble().unwrap();

        assert_eq!(first, 0x0);
        assert_eq!(*rest_entry.key, [0x1]);
        assert_eq!(*rest_entry.value, []);
    }
}
