use std::cmp::min;

/// Extracts the common prefix between two `Nibbles` and returns the remaining `Nibbles` for both inputs.
pub(crate) fn extract_common_prefix<'a>(
    lhs: &'a [u8],
    rhs: &'a [u8],
) -> (&'a [u8], &'a [u8], &'a [u8]) {
    let min_len = min(lhs.len(), rhs.len());
    let common_prefix_len = (0..min_len).take_while(|&i| lhs[i] == rhs[i]).count();

    let (common_prefix, lhs_remaining) = lhs.split_at(common_prefix_len);
    let (_, rhs_remaining) = rhs.split_at(common_prefix_len);

    (common_prefix, lhs_remaining, rhs_remaining)
}

#[cfg(test)]
mod extract_common_prefix_tests {
    use super::*;

    #[test]
    fn empty_lhs_and_rhs() {
        let lhs: &[u8] = &[];
        let rhs: &[u8] = &[];

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, &[] as &[u8]);
        assert_eq!(remaining_lhs, &[] as &[u8]);
        assert_eq!(remaining_rhs, &[] as &[u8]);
    }

    #[test]
    fn lhs_empty_rhs_non_empty() {
        let lhs: &[u8] = &[];
        let rhs: &[u8] = &[0x0];

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, &[] as &[u8]);
        assert_eq!(remaining_lhs, &[] as &[u8]);
        assert_eq!(remaining_rhs, &[0x0]);
    }

    #[test]
    fn lhs_non_empty_rhs_empty() {
        let lhs: &[u8] = &[0x0];
        let rhs: &[u8] = &[];

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, &[] as &[u8]);
        assert_eq!(remaining_lhs, &[0x0]);
        assert_eq!(remaining_rhs, &[] as &[u8]);
    }

    #[test]
    fn no_common_prefix() {
        let lhs: &[u8] = &[0x0];
        let rhs: &[u8] = &[0x1];

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, &[] as &[u8]);
        assert_eq!(remaining_lhs, &[0x0]);
        assert_eq!(remaining_rhs, &[0x1]);
    }

    #[test]
    fn one_nibble_common_prefix() {
        let lhs: &[u8] = &[0x0, 0x0];
        let rhs: &[u8] = &[0x0, 0x1];

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, &[0x0]);
        assert_eq!(remaining_lhs, &[0x0]);
        assert_eq!(remaining_rhs, &[0x1]);
    }

    #[test]
    fn two_nibbles_common_prefix() {
        let lhs: &[u8] = &[0x0, 0x0, 0x0];
        let rhs: &[u8] = &[0x0, 0x0, 0x1];

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, &[0x0, 0x0]);
        assert_eq!(remaining_lhs, &[0x0]);
        assert_eq!(remaining_rhs, &[0x1]);
    }

    #[test]
    fn identical_nibbles() {
        let lhs: &[u8] = &[0x0];
        let rhs: &[u8] = &[0x0];

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, &[0x0]);
        assert_eq!(remaining_lhs, &[] as &[u8]);
        assert_eq!(remaining_rhs, &[] as &[u8]);
    }

    #[test]
    fn different_length() {
        let lhs: &[u8] = &[0x0, 0x0];
        let rhs: &[u8] = &[0x0];

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, &[0x0]);
        assert_eq!(remaining_lhs, &[0x0]);
        assert_eq!(remaining_rhs, &[] as &[u8]);
    }
}
