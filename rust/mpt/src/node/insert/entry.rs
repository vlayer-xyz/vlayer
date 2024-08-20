use std::array::from_fn;

use nybbles::Nibbles;

use crate::{key_nibbles::KeyNibbles, node::Node};

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
    fn from(entry: Entry) -> Self {
        if entry.key.is_empty() {
            let children = from_fn(|_| None);
            Node::Branch(children, Some(entry.value))
        } else {
            Node::Leaf(KeyNibbles::from_nibbles(entry.key), entry.value)
        }
    }
}

impl Entry {
    pub fn split_first_key_nibble(self) -> Result<(u8, Entry), &'static str> {
        if let Some((first_nibble, rest)) = self.key.split_first() {
            let rest = Nibbles::from_nibbles(rest);
            Ok((
                *first_nibble,
                Entry {
                    key: rest,
                    value: self.value,
                },
            ))
        } else {
            Err("Nibbles is empty")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty_nibbles() {
        let entry = Entry {
            key: Nibbles::from_nibbles([0xA, 0xB, 0xC, 0xD]),
            value: Box::new([0x01, 0x02]),
        };

        let (first, rest_entry) = entry.split_first_key_nibble().unwrap();

        assert_eq!(first, 0xA);
        assert_eq!(rest_entry.key, Nibbles::from_nibbles([0xB, 0xC, 0xD]));
        assert_eq!(rest_entry.value.as_ref(), [0x01, 0x02]);
    }

    #[test]
    fn single_nibble() {
        let entry = Entry {
            key: Nibbles::from_nibbles([0x7]),
            value: Box::new([0x03, 0x04]),
        };

        let (first, rest_entry) = entry.split_first_key_nibble().unwrap();

        assert_eq!(first, 0x7);
        assert!(rest_entry.key.is_empty());
        assert_eq!(rest_entry.value.as_ref(), [0x03, 0x04]);
    }

    #[test]
    fn empty_nibbles() {
        let entry = Entry {
            key: Nibbles::from_nibbles([]),
            value: Box::new([0x05, 0x06]),
        };

        let result = entry.split_first_key_nibble();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Nibbles is empty");
    }
}
