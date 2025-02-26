use nybbles::Nibbles;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathKind {
    Leaf,
    Extension,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub nibbles: Nibbles,
    pub kind: PathKind,
}

impl<T> From<T> for Path
where
    T: AsRef<[u8]>,
{
    fn from(path: T) -> Self {
        let path = Nibbles::unpack(path);
        assert!(path.len() >= 2, "Path should have at least 2 nibbles");

        let kind = if path[0] & 2 != 0 {
            PathKind::Leaf
        } else {
            PathKind::Extension
        };
        let odd_nibbles = path[0] & 1 != 0;

        let prefix = if odd_nibbles {
            path[1..].to_vec()
        } else {
            path[2..].to_vec()
        };
        let nibbles = Nibbles::from_vec(prefix);
        Path { nibbles, kind }
    }
}

#[cfg(test)]
mod decode_path {
    use super::PathKind::*;
    use crate::path::Path;

    #[test]
    fn even_extension() {
        let path: Path = [0x00, 0x12].into();

        assert_eq!(path.kind, Extension);
        assert_eq!(path.nibbles.as_slice(), [0x1, 0x2]);
    }

    #[test]
    fn odd_extension() {
        let path: Path = [0x11, 0x23].into();

        assert_eq!(path.kind, Extension);
        assert_eq!(path.nibbles.as_slice(), [0x1, 0x2, 0x3]);
    }

    #[test]
    fn even_leaf() {
        let path: Path = [0x20, 0x12].into();

        assert_eq!(path.kind, Leaf);
        assert_eq!(path.nibbles.as_slice(), [0x1, 0x2]);
    }

    #[test]
    fn odd_leaf() {
        let path: Path = [0x31, 0x23].into();

        assert_eq!(path.kind, Leaf);
        assert_eq!(path.nibbles.as_slice(), [0x1, 0x2, 0x3]);
    }

    #[test]
    #[should_panic(expected = "Path should have at least 2 nibbles")]
    fn too_short() {
        let _: Path = [].into();
    }
}
