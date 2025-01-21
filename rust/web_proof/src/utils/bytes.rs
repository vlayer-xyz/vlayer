pub(crate) fn replace_bytes(input: &[u8], from: u8, to: u8) -> Vec<u8> {
    input
        .iter()
        .map(|&b| if b == from { to } else { b })
        .collect()
}

pub(crate) fn all_match(input: &[u8], target: u8) -> bool {
    input.iter().all(|&c| c == target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_bytes() {
        // Basic replacement
        assert_eq!(replace_bytes(b"hello", b'e', b'a'), b"hallo");

        // Multiple occurrences
        assert_eq!(replace_bytes(b"test test test", b't', b'T'), b"TesT TesT TesT");

        // No replacements needed
        assert_eq!(replace_bytes(b"nochange", b'x', b'z'), b"nochange");

        // All bytes replaced
        assert_eq!(replace_bytes(b"aaaa", b'a', b'b'), b"bbbb");

        // Empty input
        assert_eq!(replace_bytes(b"", b'a', b'b'), b"");

        // Replacing null bytes
        assert_eq!(replace_bytes(&[0, 1, 0, 2], 0, 255), &[255, 1, 255, 2]);
    }

    #[test]
    fn test_all_match() {
        // All elements match
        assert!(all_match(b"aaaa", b'a'));

        // Some elements do not match
        assert!(!all_match(b"aaab", b'a'));

        // Single-element cases
        assert!(all_match(b"x", b'x'));
        assert!(!all_match(b"x", b'y'));

        // Empty input
        assert!(all_match(b"", b'a'));
    }
}
