fn longest_common_prefix_len(lhs: &[u8], rhs: &[u8]) -> usize {
    lhs.iter()
        .zip(rhs.iter())
        .take_while(|(lhs, rhs)| lhs == rhs)
        .count()
}

pub(crate) fn extract_common_prefix<'a>(
    lhs: &'a [u8],
    rhs: &'a [u8],
) -> (&'a [u8], &'a [u8], &'a [u8]) {
    let len = longest_common_prefix_len(lhs, rhs);
    let (prefix, lhs_remaining) = lhs.split_at(len);
    let (_, rhs_remaining) = rhs.split_at(len);
    (prefix, lhs_remaining, rhs_remaining)
}

#[cfg(test)]
mod longest_common_prefix_len {
    use super::*;

    #[test]
    fn both_empty() {
        assert_eq!(longest_common_prefix_len(&[], &[]), 0);
    }

    #[test]
    fn one_empty() {
        assert_eq!(longest_common_prefix_len(&[0], &[]), 0);
        assert_eq!(longest_common_prefix_len(&[], &[0]), 0);
    }

    #[test]
    fn no_common() {
        assert_eq!(longest_common_prefix_len(&[0], &[1]), 0);
    }

    #[test]
    fn common_prefix() {
        assert_eq!(longest_common_prefix_len(&[0, 1], &[0, 2]), 1);
    }
}

#[cfg(test)]
mod extract_common_prefix {
    use super::*;

    #[test]
    fn success() {
        let (prefix, lhs, rhs) = extract_common_prefix(&[0, 1], &[0, 2]);
        assert_eq!(prefix, [0]);
        assert_eq!(lhs, [1]);
        assert_eq!(rhs, [2]);
    }
}
