use crate::node::{constructors::EMPTY_CHILDREN, Node};

use super::entry::Entry;

fn from_extension_and_entry(extension: Node, entry: Entry) {
    if let Node::Extension(key, node) = extension {
        let (entry_first_nibble, remaining_entry) = entry.split_first_key_nibble();

        if key[0] != entry_first_nibble {
            let mut children = EMPTY_CHILDREN.clone();

        }

    } else {
        unreachable!("from_extension_and_entry is used only for Extension nodes");
    }
}

fn pop_extension_key(extension: Node) -> (u8, Node) {
    if let Node::Extension(key, node) = extension {
        (key[0], *node)
    } else {
        unreachable!("pop_extension_key is used only for Extension nodes");
    }
}
