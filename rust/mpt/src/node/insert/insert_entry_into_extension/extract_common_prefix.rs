use nybbles::Nibbles;

/// Extracts the common prefix between two `Nibbles` and returns the remaining `Nibbles` for both inputs.
pub(crate) fn extract_common_prefix(lhs: Nibbles, rhs: Nibbles) -> (Nibbles, Nibbles, Nibbles) {
    let mut common_prefix = Nibbles::default();
    let min_len = lhs.len().min(rhs.len());

    for i in 0..min_len {
        if lhs[i] == rhs[i] {
            common_prefix.push(lhs[i]);
        } else {
            break;
        }
    }

    let (_, lhs_remaining) = lhs.split_at(common_prefix.len());
    let (_, rhs_remaining) = rhs.split_at(common_prefix.len());

    (
        common_prefix,
        Nibbles::from_nibbles(lhs_remaining),
        Nibbles::from_nibbles(rhs_remaining),
    )
}

#[cfg(test)]
mod extract_common_prefix_tests {
    use super::*;

    #[test]
    fn empty_lhs_and_rhs() {
        let lhs = Nibbles::from_vec(vec![]);
        let rhs = Nibbles::from_vec(vec![]);

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, Nibbles::from_vec(vec![]));
        assert_eq!(remaining_lhs, Nibbles::from_vec(vec![]));
        assert_eq!(remaining_rhs, Nibbles::from_vec(vec![]));
    }

    #[test]
    fn lhs_empty_rhs_non_empty() {
        let lhs = Nibbles::from_vec(vec![]);
        let rhs = Nibbles::from_vec(vec![0x0]);

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, Nibbles::from_vec(vec![]));
        assert_eq!(remaining_lhs, Nibbles::from_vec(vec![]));
        assert_eq!(remaining_rhs, Nibbles::from_vec(vec![0x0]));
    }

    #[test]
    fn lhs_non_empty_rhs_empty() {
        let lhs = Nibbles::from_vec(vec![0x0]);
        let rhs = Nibbles::from_vec(vec![]);

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, Nibbles::from_vec(vec![]));
        assert_eq!(remaining_lhs, Nibbles::from_vec(vec![0x0]));
        assert_eq!(remaining_rhs, Nibbles::from_vec(vec![]));
    }

    #[test]
    fn no_common_prefix() {
        let lhs = Nibbles::from_vec(vec![0x0]);
        let rhs = Nibbles::from_vec(vec![0x1]);

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, Nibbles::from_vec(vec![]));
        assert_eq!(remaining_lhs, Nibbles::from_vec(vec![0x0]));
        assert_eq!(remaining_rhs, Nibbles::from_vec(vec![0x1]));
    }

    #[test]
    fn one_nibble_common_prefix() {
        let lhs = Nibbles::from_vec(vec![0x0, 0x0]);
        let rhs = Nibbles::from_vec(vec![0x0, 0x1]);

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, Nibbles::from_vec(vec![0x0]));
        assert_eq!(remaining_lhs, Nibbles::from_vec(vec![0x0]));
        assert_eq!(remaining_rhs, Nibbles::from_vec(vec![0x1]));
    }

    #[test]
    fn two_nibbles_common_prefix() {
        let lhs = Nibbles::from_vec(vec![0x0, 0x0, 0x0]);
        let rhs = Nibbles::from_vec(vec![0x0, 0x0, 0x1]);

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, Nibbles::from_vec(vec![0x0, 0x0]));
        assert_eq!(remaining_lhs, Nibbles::from_vec(vec![0x0]));
        assert_eq!(remaining_rhs, Nibbles::from_vec(vec![0x1]));
    }

    #[test]
    fn identical_nibbles() {
        let lhs = Nibbles::from_vec(vec![0x0]);
        let rhs = Nibbles::from_vec(vec![0x0]);

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, Nibbles::from_vec(vec![0x0]));
        assert_eq!(remaining_lhs, Nibbles::from_vec(vec![]));
        assert_eq!(remaining_rhs, Nibbles::from_vec(vec![]));
    }

    #[test]
    fn different_length() {
        let lhs = Nibbles::from_vec(vec![0x0, 0x0]);
        let rhs = Nibbles::from_vec(vec![0x0]);

        let (common, remaining_lhs, remaining_rhs) = extract_common_prefix(lhs, rhs);

        assert_eq!(common, Nibbles::from_vec(vec![0x0]));
        assert_eq!(remaining_lhs, Nibbles::from_vec(vec![0x0]));
        assert_eq!(remaining_rhs, Nibbles::from_vec(vec![]));
    }
}
