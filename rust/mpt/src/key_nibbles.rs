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

    pub fn push_front(&self, nibble: u8) -> Self {
        let mut nibbles = vec![nibble];
        nibbles.extend_from_slice(self);
        KeyNibbles(Nibbles::from_vec(nibbles))
    }

    pub fn split_first(&self) -> (u8, &[u8]) {
        let first = self.0[0];
        let rest = &self.0[1..];
        (first, rest)
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

#[cfg(test)]
mod push_front {
    use super::*;

    #[test]
    fn single_nibble() {
        let original: KeyNibbles = [0x1, 0x2].into();
        let result = original.push_front(0x0);
        let expected: KeyNibbles = [0x0, 0x1, 0x2].into();
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_times() {
        let original: KeyNibbles = [0x2].into();
        let result = original.push_front(0x1).push_front(0x0);
        let expected: KeyNibbles = [0x0, 0x1, 0x2].into();
        assert_eq!(result, expected);
    }
}
