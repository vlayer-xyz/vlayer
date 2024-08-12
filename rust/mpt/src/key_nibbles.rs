use derive_more::Deref;

use nybbles::Nibbles;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deref, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyNibbles(Nibbles);

impl KeyNibbles {
    pub fn new<T: AsRef<[u8]>>(input: T) -> Self {
        let nibbles = Nibbles::from_nibbles(input);
        Self::from_nibbles(nibbles)
    }

    pub fn unpack<T: AsRef<[u8]>>(data: T) -> Self {
        let nibbles = Nibbles::unpack(data);
        Self::from_nibbles(nibbles)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    fn from_nibbles(nibbles: Nibbles) -> Self {
        if nibbles.is_empty() {
            panic!("KeyNibbles cannot be empty");
        }
        KeyNibbles(nibbles)
    }
}

impl PartialEq<[u8]> for KeyNibbles {
    fn eq(&self, other: &[u8]) -> bool {
        self.0.as_slice() == other
    }
}

#[cfg(test)]
mod new {
    use super::*;

    #[test]
    fn non_empty() {
        let valid_nibbles = vec![0x1, 0x2, 0x3];
        let key_nibbles = KeyNibbles::new(&valid_nibbles);

        assert_eq!(key_nibbles, valid_nibbles[..]);
    }

    #[test]
    #[should_panic(expected = "KeyNibbles cannot be empty")]
    fn empty() {
        let empty_nibbles = vec![];
        let _ = KeyNibbles::new(&empty_nibbles);
    }
}
