use self::{entry::Entry, from_two_entries::from_two_entries};
use super::{Node, NodeError};

mod entry;
mod from_two_entries;
mod insert_entry_into_branch;
mod insert_entry_into_extension;
mod tests;
mod utils;

impl<D> Node<D> {
    #[allow(clippy::panic)]
    pub(crate) fn insert(
        self,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> Result<Node<D>, NodeError> {
        let key = key.as_ref();
        match self {
            Node::Null => Ok(Entry::from((key, value)).into()),
            Node::Digest(_) => panic!("Cannot insert into a digest node"),
            Node::Leaf(old_key, old_value) => {
                let old_entry = Entry::new(old_key, old_value);
                let entry = (key, value);
                from_two_entries(old_entry, entry)
            }
            Node::Branch(_, _) => self.insert_entry_into_branch((key, value)),
            Node::Extension(_, _) => self.insert_entry_into_extension((key, value)),
            Node::_Phantom(_) => unreachable!(),
        }
    }
}
