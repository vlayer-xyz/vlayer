use nybbles::Nibbles;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyNibbles(Nibbles);

impl KeyNibbles {
    pub fn new(nibbles: Nibbles) -> Result<Self, &'static str> {
        if nibbles.is_empty() {
            return Err("KeyNibbles cannot be empty");
        }
        Ok(KeyNibbles(nibbles))
    }

    pub fn as_nibbles(&self) -> &Nibbles {
        &self.0
    }
}

impl TryFrom<Nibbles> for KeyNibbles {
    type Error = &'static str;

    fn try_from(nibbles: Nibbles) -> Result<Self, Self::Error> {
        KeyNibbles::new(nibbles)
    }
}

impl From<KeyNibbles> for Nibbles {
    fn from(key_nibbles: KeyNibbles) -> Self {
        key_nibbles.0
    }
}

#[cfg(test)]
mod key_nibbles {
    use super::*;
    use nybbles::Nibbles;

    #[test]
    fn creation_success() {
        let valid_nibbles = Nibbles::from_vec(vec![0x1, 0x2, 0x3]);
        let key_nibbles = KeyNibbles::new(valid_nibbles.clone());

        assert!(key_nibbles.is_ok());
        assert_eq!(key_nibbles.unwrap().as_nibbles(), &valid_nibbles);
    }

    #[test]
    fn creation_failure_empty() {
        let empty_nibbles = Nibbles::from_vec(vec![]);
        let key_nibbles = KeyNibbles::new(empty_nibbles);

        assert!(key_nibbles.is_err());
        assert_eq!(key_nibbles.err(), Some("KeyNibbles cannot be empty"));
    }

    #[test]
    fn test_try_from_nibbles_success() {
        let valid_nibbles = Nibbles::from_vec(vec![0x4, 0x5]);
        let key_nibbles = KeyNibbles::try_from(valid_nibbles.clone());

        assert!(key_nibbles.is_ok());
        assert_eq!(key_nibbles.unwrap().as_nibbles(), &valid_nibbles);
    }

    #[test]
    fn test_try_from_nibbles_failure_empty() {
        let empty_nibbles = Nibbles::from_vec(vec![]);
        let key_nibbles = KeyNibbles::try_from(empty_nibbles);

        assert!(key_nibbles.is_err());
        assert_eq!(key_nibbles.err(), Some("KeyNibbles cannot be empty"));
    }

    #[test]
    fn test_key_nibbles_to_nibbles() {
        let valid_nibbles = Nibbles::from_vec(vec![0x6, 0x7]);
        let key_nibbles = KeyNibbles::new(valid_nibbles.clone()).unwrap();
        let nibbles: Nibbles = key_nibbles.into();

        assert_eq!(nibbles, valid_nibbles);
    }
}
