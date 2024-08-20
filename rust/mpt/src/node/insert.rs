use self::entry::Entry;

use super::Node;
use nybbles::Nibbles;

mod entry;

impl Node {
    #[allow(unused)]
    pub(crate) fn insert(self, key: Nibbles, value: impl AsRef<[u8]>) -> Node {
        match self {
            Node::Null => Node::insert_into_null(Entry {
                key,
                value: value.as_ref().into(),
            }),
            Node::Leaf(old_key, old_value) => {
                let old_entry = (&**old_key, old_value).into();
                let entry = (&*key, value).into();
                Node::insert_entry(old_entry, entry)
            }
            _ => panic!("Not implemented"),
        }
    }

    fn insert_into_null(entry: Entry) -> Node {
        if entry.key.is_empty() {
            Node::Branch(Default::default(), Some(entry.value.as_ref().into()))
        } else {
            Node::create_leaf(entry.key.as_slice(), entry.value)
        }
    }

    fn insert_entry(old_entry: Entry, entry: Entry) -> Node {
        if old_entry.key == entry.key {
            panic!("Key already exists");
        } else {
            let (first_old_key_nibble, remaining_old_entry) =
                old_entry.split_first_key_nibble().unwrap();
            let (first_key_nibble, remaining_entry) = entry.split_first_key_nibble().unwrap();
            if first_old_key_nibble != first_key_nibble {
                let mut children: [Option<Box<Node>>; 16] = Default::default();
                children[first_old_key_nibble as usize] =
                    Some(Box::new(remaining_old_entry.into()));
                children[first_key_nibble as usize] = Some(Box::new(remaining_entry.into()));
                Node::Branch(children, None)
            } else {
                todo!("Extend with branch or extension");
            }
        }
    }
}
