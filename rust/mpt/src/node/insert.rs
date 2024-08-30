use self::entry::Entry;
use self::from_two_entries::from_two_entries;

use super::{Node, NodeError};
use nybbles::Nibbles;

mod entry;
mod from_two_entries;
mod insert_entry_into_branch;
mod tests;
mod utils;

impl Node {
    #[allow(unused)]
    pub(crate) fn insert(self, key: Nibbles, value: impl AsRef<[u8]>) -> Result<Node, NodeError> {
        match self {
            Node::Null => Ok(Entry::from((&*key, value)).into()),
            Node::Digest(_) => panic!("Cannot insert into a digest node"),
            Node::Leaf(old_key, old_value) => {
                let old_entry = (&**old_key, old_value).into();
                let entry = (&*key, value).into();
                from_two_entries(old_entry, entry)
            }
            Node::Branch(_, _) => self.insert_entry_into_branch((&*key, value)),
            _ => todo!("Implement insert for Extension"),
        }
    }
}
