use derive_more::Deref;

use nybbles::Nibbles;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deref, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyNibbles(Nibbles);

impl KeyNibbles {
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

impl<T: AsRef<[u8]>> PartialEq<T> for KeyNibbles {
    fn eq(&self, other: &T) -> bool {
        self.0.as_slice() == other.as_ref()
    }
}

impl<T: AsRef<[u8]>> From<T> for KeyNibbles {
    fn from(input: T) -> Self {
        let nibbles = Nibbles::from_nibbles(input);
        Self::from_nibbles(nibbles)
    }
}

#[cfg(test)]
mod new {
    use super::*;

    #[test]
    fn non_empty() {
        let key_nibbles: KeyNibbles = [0x0].into();
        assert_eq!(key_nibbles, [0x0]);
    }

    #[test]
    #[should_panic(expected = "KeyNibbles cannot be empty")]
    fn empty() {
        let _ = KeyNibbles::from([]);
    }
}
