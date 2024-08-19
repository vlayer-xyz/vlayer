use crate::key_nibbles::KeyNibbles;

use super::Node;
use nybbles::Nibbles;

impl Node {
    #[allow(unused)]
    pub(crate) fn insert(&mut self, key: Nibbles, value: impl AsRef<[u8]>) {
        let nibble = key.clone().pop();
        match self {
            Node::Null => {
                self.insert_null(key, value);
            }
            Node::Leaf(old_key, old_value) => {
                let old_key = old_key.clone();
                let old_value = old_value.clone();
                self.insert_into_leaf(&old_key, &old_value, key, value);
            }
            _ => {}
        }
    }

    fn insert_null(&mut self, key: Nibbles, value: impl AsRef<[u8]>) {
        if key.is_empty() {
            *self = Node::Branch(Default::default(), Some(value.as_ref().into()));
        } else {
            *self = Node::create_leaf(key.as_slice(), value);
        }
    }

    fn insert_into_leaf(
        &mut self,
        old_key: &Nibbles,
        old_value: &[u8],
        key: Nibbles,
        value: impl AsRef<[u8]>,
    ) {
        if **old_key == key {
            panic!("Key already exists");
        } else {
            let (old_key_first_nibble, remaining_old_key) = split_first_nibble(old_key).unwrap();
            let (key_first_nibble, remaining_key) = split_first_nibble(&key).unwrap();

            let mut children: [Option<Box<Node>>; 16] = Default::default();
            children[old_key_first_nibble as usize] = Some(Box::new(Node::Leaf(
                KeyNibbles::from_nibbles(remaining_old_key),
                old_value.into(),
            )));
            children[key_first_nibble as usize] = Some(Box::new(Node::Leaf(
                KeyNibbles::from_nibbles(remaining_key),
                value.as_ref().into(),
            )));

            *self = Node::Branch(children, None);
        }
    }
}

pub fn split_first_nibble(nibbles: &Nibbles) -> Result<(u8, Nibbles), &'static str> {
    if let Some(splitted) = nibbles.split_first() {
        let first_nibble = Nibbles::from_nibbles([*splitted.0]);
        let rest = Nibbles::from_nibbles(splitted.1);
        Ok((first_nibble[0], rest))
    } else {
        Err("Nibbles is empty")
    }
}

#[cfg(test)]
mod split_first_nibble {
    use super::*;

    #[test]
    fn non_empty_nibbles() {
        let nibbles = Nibbles::from_nibbles([0xA, 0xB, 0xC, 0xD]);
        let result = split_first_nibble(&nibbles);

        assert!(result.is_ok());
        let (first, rest) = result.unwrap();

        assert_eq!(first, 0xA);
        assert_eq!(rest, Nibbles::from_nibbles([0xB, 0xC, 0xD]));
    }

    #[test]
    fn single_nibble() {
        let nibbles = Nibbles::from_nibbles([0x7]);
        let result = split_first_nibble(&nibbles);

        assert!(result.is_ok());
        let (first, rest) = result.unwrap();

        assert_eq!(first, 0x7);
        assert!(rest.is_empty());
    }

    #[test]
    fn empty_nibbles() {
        let nibbles = Nibbles::from_nibbles([]);
        let result = split_first_nibble(&nibbles);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Nibbles is empty");
    }
}
