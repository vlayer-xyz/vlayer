#[cfg(test)]
mod split_first_nibble {
    use nybbles::Nibbles;

    use crate::node::insert::split_first_nibble;

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
        let nibbles: Nibbles = Nibbles::from_nibbles([0x7]);
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
