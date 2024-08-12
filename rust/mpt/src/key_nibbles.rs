use std::ops::{Deref, Index, RangeFrom};

use nybbles::Nibbles;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyNibbles(Nibbles);

impl KeyNibbles {
    pub fn new<T: AsRef<[u8]>>(input: T) -> Self {
        let nibbles = Nibbles::from_vec(input.as_ref().to_vec());
        Self::create_and_validate(nibbles).unwrap()
    }

    fn create_and_validate(nibbles: Nibbles) -> Result<Self, &'static str> {
        if nibbles.is_empty() {
            panic!("KeyNibbles cannot be empty");
        }
        Ok(KeyNibbles(nibbles))
    }

    pub fn unpack<T: AsRef<[u8]>>(data: T) -> Self {
        let nibbles = Nibbles::unpack(data);
        Self::create_and_validate(nibbles).unwrap()
    }

    pub fn as_nibbles(&self) -> &Nibbles {
        &self.0
    }
}

impl Deref for KeyNibbles {
    type Target = Nibbles;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<KeyNibbles> for Nibbles {
    fn from(key_nibbles: KeyNibbles) -> Self {
        key_nibbles.0
    }
}

impl PartialEq<[u8]> for KeyNibbles {
    fn eq(&self, other: &[u8]) -> bool {
        self.0.as_slice() == other
    }
}

impl Index<usize> for KeyNibbles {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0.as_slice()[index]
    }
}

impl Index<RangeFrom<usize>> for KeyNibbles {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0.as_slice()[index]
    }
}

#[cfg(test)]
mod key_nibbles {
    use super::*;
    use nybbles::Nibbles;

    #[test]
    fn creation_success() {
        let valid_nibbles = vec![0x1, 0x2, 0x3];
        let key_nibbles = KeyNibbles::new(&valid_nibbles);

        assert_eq!(key_nibbles.as_nibbles(), &valid_nibbles[..]);
    }

    #[test]
    #[should_panic(expected = "KeyNibbles cannot be empty")]
    fn creation_failure_empty() {
        let empty_nibbles = vec![];
        let _ = KeyNibbles::new(&empty_nibbles);
    }

    #[test]
    fn try_from_vec_success() {
        let valid_nibbles = vec![0x4, 0x5];
        let key_nibbles = KeyNibbles::new(&valid_nibbles);

        assert_eq!(key_nibbles.as_nibbles(), &valid_nibbles[..]);
    }

    #[test]
    #[should_panic(expected = "KeyNibbles cannot be empty")]
    fn try_from_vec_failure_empty() {
        let empty_nibbles = vec![];
        let _ = KeyNibbles::new(&empty_nibbles);
    }

    #[test]
    fn key_nibbles_to_nibbles() {
        let valid_nibbles = vec![0x6, 0x7];
        let key_nibbles = KeyNibbles::new(&valid_nibbles);
        let nibbles: Nibbles = key_nibbles.into();

        assert_eq!(nibbles.as_slice(), &valid_nibbles[..]);
    }
}
