use self::entry::Entry;
use self::from_two_entries::from_two_entries;

use super::Node;
use nybbles::Nibbles;

mod entry;
mod from_two_entries;
mod tests;

impl Node {
    #[allow(unused)]
    pub(crate) fn insert(self, key: Nibbles, value: impl AsRef<[u8]>) -> Node {
        match self {
            Node::Null => Entry::from((&*key, value)).into(),
            Node::Leaf(old_key, old_value) => {
                let old_entry = (&**old_key, old_value).into();
                let entry = (&*key, value).into();
                from_two_entries(old_entry, entry)
            }
            _ => todo!("Branch and extension nodes"),
        }
    }
}
