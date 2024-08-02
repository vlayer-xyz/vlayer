use nybbles::Nibbles;

#[inline]
pub fn decode_path(path: impl AsRef<[u8]>) -> (Nibbles, bool) {
    let path = Nibbles::unpack(path);
    assert!(path.len() >= 2);

    let is_leaf = path[0] & 2 != 0;
    let odd_nibbles = path[0] & 1 != 0;

    let prefix = if odd_nibbles { &path[1..] } else { &path[2..] };
    (Nibbles::from_nibbles_unchecked(prefix), is_leaf)
}

#[cfg(test)]
mod decode_path {
    #[test]
    fn even_extension() {
        let (path, is_leaf) = super::decode_path(&[0x00, 0x12]);

        assert!(!is_leaf);
        assert_eq!(path.as_slice(), &[0x1, 0x2][..]);
    }

    #[test]
    fn odd_extension() {
        let (path, is_leaf) = super::decode_path(&[0x11, 0x23]);

        assert!(!is_leaf);
        assert_eq!(path.as_slice(), &[0x1, 0x2, 0x3][..]);
    }

    #[test]
    fn even_leaf() {
        let (path, is_leaf) = super::decode_path(&[0x20, 0x12]);

        assert!(is_leaf);
        assert_eq!(path.as_slice(), &[0x1, 0x2][..]);
    }

    #[test]
    fn odd_leaf() {
        let (path, is_leaf) = super::decode_path(&[0x31, 0x23]);

        assert!(is_leaf);
        assert_eq!(path.as_slice(), &[0x1, 0x2, 0x3][..]);
    }
}
