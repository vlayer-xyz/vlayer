use self::entry::Entry;

use super::Node;
use nybbles::Nibbles;

mod entry;

impl Node {
    #[allow(unused)]
    pub(crate) fn insert(self, key: Nibbles, value: impl AsRef<[u8]>) -> Node {
        let nibble = key.clone().pop();
        match self {
            Node::Null => Node::insert_null(key, value),
            Node::Leaf(old_key, old_value) => {
                let old_entry = Entry {
                    key: (*old_key).clone(),
                    value: old_value.into(),
                };
                let entry = Entry {
                    key,
                    value: value.as_ref().into(),
                };
                Node::insert_into_leaf(old_entry, entry)
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

    fn insert_into_leaf(old_entry: Entry, entry: Entry) -> Node {
        if old_entry.key == entry.key {
            panic!("Key already exists");
        } else {
            let (old_key_first_nibble, remaining_old_entry) =
                old_entry.split_first_key_nibble().unwrap();
            let (key_first_nibble, remaining_entry) = entry.split_first_key_nibble().unwrap();

            let mut children: [Option<Box<Node>>; 16] = Default::default();
            children[old_key_first_nibble as usize] = Some(Box::new(remaining_old_entry.into()));
            children[key_first_nibble as usize] = Some(Box::new(remaining_entry.into()));
            Node::Branch(children, None)
        }
    }
}
