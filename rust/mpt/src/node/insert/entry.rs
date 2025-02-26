// The (key, value) pair is often represented in the Node::insert logic using the Entry structure.
// The Entry struct provides a concise way to encapsulate both the key and the value, allowing them
// to be handled as a single unit, thus reducing code clutter and unnecessary conversions.

use alloy_primitives::Bytes;
use derive_new::new;
use nybbles::Nibbles;

use crate::node::Node;

#[derive(Debug, Clone, PartialEq, new)]
pub struct Entry {
    pub key: Nibbles,
    pub value: Bytes,
}

// From<(K, V)> for Entry implementation converts a (key, value) tuple into an Entry struct.
// This allows helper functions like insert_entry_into_branch and insert_entry_into_extension
// to accept a tuple directly, simplifying their interfaces.
impl<K, V> From<(K, V)> for Entry
where
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
{
    fn from((key, value): (K, V)) -> Self {
        Entry {
            key: Nibbles::from_nibbles(key.as_ref()),
            value: Bytes::copy_from_slice(value.as_ref()),
        }
    }
}

// From<Entry> for Node implementation converts an Entry into a Node,
// creating a Branch if the key is empty or a Leaf if it is not.
impl<D> From<Entry> for Node<D> {
    fn from(Entry { key, value }: Entry) -> Self {
        Node::leaf(&*key, value)
    }
}

impl Entry {
    // Splits the first nibble from the entry's key, returning it along with the remaining entry.
    pub(crate) fn split_first_key_nibble(self) -> (u8, Entry) {
        let Some((first_key_nibble, remaining_key)) = self.key.split_first() else {
            unreachable!("Can't split first key nibble from empty nibbles");
        };
        let entry = (remaining_key, self.value).into();
        (*first_key_nibble, entry)
    }
}

#[cfg(test)]
mod split_first_key_nibble {
    use super::*;

    #[test]
    #[should_panic(expected = "Can't split first key nibble from empty nibbles")]
    fn empty_nibbles() {
        let entry: Entry = ([], []).into();
        entry.split_first_key_nibble();
    }

    #[test]
    fn single_nibble() {
        let entry: Entry = ([0x0], []).into();

        let (first, rest_entry) = entry.split_first_key_nibble();

        assert_eq!(first, 0x0);
        let value: &[u8] = &rest_entry.value;
        assert_eq!(value, [] as [u8; 0]);
    }

    #[test]
    fn non_empty_nibbles() {
        let entry: Entry = ([0x0, 0x1], []).into();

        let (first, rest_entry) = entry.split_first_key_nibble();

        assert_eq!(first, 0x0);
        let key: &[u8] = &rest_entry.key;
        let value: &[u8] = &rest_entry.value;
        assert_eq!(key, [0x1]);
        assert_eq!(value, [] as [u8; 0]);
    }
}
