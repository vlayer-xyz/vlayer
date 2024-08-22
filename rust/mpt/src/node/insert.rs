use self::entry::Entry;

use super::Node;
use nybbles::Nibbles;

mod entry;

impl Node {
    #[allow(unused)]
    pub(crate) fn insert(self, key: Nibbles, value: impl AsRef<[u8]>) -> Node {
        match self {
            Node::Null => Entry::from((&*key, value)).into(),
            Node::Leaf(old_key, old_value) => {
                let old_entry = (&**old_key, old_value).into();
                let entry = (&*key, value).into();
                Node::insert_entry(old_entry, entry)
            }
            _ => panic!("Not implemented"),
        }
    }

    fn insert_entry(old_entry: Entry, entry: Entry) -> Node {
        if old_entry.key == entry.key {
            panic!("Key already exists");
        } else {
            if old_entry.key.is_empty() || entry.key.is_empty() {
                todo!("Handle empty key case");
            }
            let (first_old_key_nibble, remaining_old_entry) = old_entry.split_first_key_nibble();
            let (first_key_nibble, remaining_entry) = entry.split_first_key_nibble();
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
