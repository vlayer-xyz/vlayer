#![allow(unused)]
pub fn key_values(
    header: &str,
) -> impl Iterator<Item = Result<(String, String), &'static str>> + '_ {
    header.split(';').filter_map(parse_key_value_pair)
}

fn parse_key_value_pair(pair: &str) -> Option<Result<(String, String), &'static str>> {
    if pair.trim().is_empty() {
        return None;
    }
    let mut key_value = pair.splitn(2, '=');
    let key = match key_value.next() {
        Some(key) => key.trim(),
        None => return Some(Err("Missing key")),
    };
    let value = match key_value.next() {
        Some(value) => value.trim(),
        None => return Some(Err("Missing value")),
    };
    if key.is_empty() {
        return Some(Err("Missing key"));
    }
    Some(Ok((key.to_string(), value.to_string())))
}

#[cfg(test)]
mod test {
    use super::*;

    mod key_values {
        use super::*;

        #[test]
        fn splits_key_value_pairs() {
            let mut key_values = key_values("key1=value1;key2=value2");
            assert_eq!(key_values.next(), Some(Ok(("key1".to_string(), "value1".to_string()))));
            assert_eq!(key_values.next(), Some(Ok(("key2".to_string(), "value2".to_string()))));
            assert_eq!(key_values.next(), None);
        }

        #[test]
        fn removes_whitespaces_around_tags() {
            let mut key_values = key_values(" \t key1\t= \tvalue1 ; key2 =\t value2\t\t");
            assert_eq!(key_values.next(), Some(Ok(("key1".to_string(), "value1".to_string()))));
            assert_eq!(key_values.next(), Some(Ok(("key2".to_string(), "value2".to_string()))));
            assert_eq!(key_values.next(), None);
        }

        #[test]
        fn skips_empty_tags() {
            let mut key_values = key_values(";key1=value1;;;key2=value2;");
            assert_eq!(key_values.next(), Some(Ok(("key1".to_string(), "value1".to_string()))));
            assert_eq!(key_values.next(), Some(Ok(("key2".to_string(), "value2".to_string()))));
            assert_eq!(key_values.next(), None);
        }

        #[test]
        fn returns_error_for_invalid_text() {
            let mut key_values = key_values("a=b;=value");
            assert_eq!(key_values.next(), Some(Ok(("a".to_string(), "b".to_string()))));
            assert_eq!(key_values.next(), Some(Err("Missing key")));
            assert_eq!(key_values.next(), None);
        }
    }

    mod parse_key_value_pair {
        use super::*;

        #[test]
        fn splits_key_and_value() {
            let (key, value) = parse_key_value_pair("key=value").unwrap().unwrap();
            assert_eq!(key, "key");
            assert_eq!(value, "value");
        }

        #[test]
        fn trims_key_and_value() {
            let (key, value) = parse_key_value_pair(" key = value ").unwrap().unwrap();
            assert_eq!(key, "key");
            assert_eq!(value, "value");
        }

        #[test]
        fn trims_key_and_value2() {
            let (key, value) = parse_key_value_pair(" key = value ").unwrap().unwrap();
            assert_eq!(key, "key");
            assert_eq!(value, "value");
        }

        #[test]
        fn returns_error_on_missing_key() {
            assert_eq!(parse_key_value_pair("=value"), Some(Err("Missing key")));
        }

        #[test]
        fn returns_error_on_missing_eq_sign() {
            assert_eq!(parse_key_value_pair("value"), Some(Err("Missing value")));
        }

        #[test]
        fn works_for_missing_value() {
            assert_eq!(parse_key_value_pair("key="), Some(Ok(("key".to_string(), "".to_string()))));
        }
    }
}
