use ::regex::Regex;
use alloy_primitives::Bytes;
use alloy_sol_types::{SolType, SolValue, sol_data};
use regex::{Captures, Match, RegexBuilder};

use crate::helpers::{Result, map_to_fatal};

const REGEX_SIZE_LIMIT: usize = 1_000_000;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

pub(super) fn is_match(input: &Bytes) -> Result<Bytes> {
    let (source, regex) = decode_args(input)?;
    Ok(regex.is_match(&source).abi_encode().into())
}

pub(super) fn capture(input: &Bytes) -> Result<Bytes> {
    let (source, regex) = decode_args(input)?;
    do_capture(&source, &regex)
        .map(|x| x.abi_encode().into())
        .map_err(map_to_fatal)
}

fn do_capture(source: &str, regex: &Regex) -> std::result::Result<Vec<String>, &'static str> {
    regex
        .captures(source)
        .as_ref()
        .map(captures_to_strings)
        .ok_or("No match found")
}

fn captures_to_strings(captures: &Captures) -> Vec<String> {
    captures.iter().map(match_into_string).collect()
}

fn match_into_string(maybe_match: Option<Match>) -> String {
    match maybe_match {
        None => "".into(),
        Some(m) => m.as_str().into(),
    }
}

fn decode_args(input: &Bytes) -> Result<(String, Regex)> {
    let [source, pattern] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;
    validate_regex(&pattern).map_err(map_to_fatal)?;
    let regex = RegexBuilder::new(&pattern)
        .size_limit(REGEX_SIZE_LIMIT)
        .dfa_size_limit(REGEX_SIZE_LIMIT)
        .build()
        .map_err(map_to_fatal)?;
    Ok((source, regex))
}

fn validate_regex(pattern: &str) -> std::result::Result<(), &'static str> {
    if !(pattern.starts_with('^') && pattern.ends_with('$')) {
        return Err(r#"Regex must be surrounded by "^" and "$" pair to match the whole string"#);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use revm::precompile::PrecompileErrors;

    use super::*;

    mod match_test {
        use super::*;

        #[test]
        fn test_regex_match() {
            let source = "Hello, World!";
            let regex = r"^Hello, World!$";

            let input = [source, regex].abi_encode();

            let result = is_match(&Bytes::from(input)).unwrap();
            let result = bool::abi_decode(&result, true).unwrap();

            assert!(result);
        }

        #[test]
        fn regex_has_max_size_limit() {
            let source = "Hello, World!";
            let regex = r"^Hello, World!\w{100}$";

            let input = [source, regex].abi_encode();

            let result = is_match(&Bytes::from(input));
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: "Compiled regex exceeds size limit of 1000000 bytes.".into()
                })
            );
        }

        #[test]
        fn regex_size_can_be_optimized() {
            let source = "Hello, World!";
            let regex = r"^Hello, World![a-zA-Z0-9]{10000}$";

            let input = [source, regex].abi_encode();

            let result = is_match(&Bytes::from(input));

            assert!(result.is_ok());
        }

        #[test]
        fn test_regex_no_match() {
            let source = "Hello, World!";
            let regex = r"^Goodbye, World!$";

            let input = [source, regex].abi_encode();

            let result = is_match(&Bytes::from(input)).unwrap();
            let result = bool::abi_decode(&result, true).unwrap();

            assert!(!result);
        }

        #[test]
        fn returns_error_when_invalid_regex() {
            let source = "Hello, World!";
            let regex = r"^[$";

            let input = [source, regex].abi_encode();

            let result = is_match(&Bytes::from(input));
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: "regex parse error:\n    ^[$\n     ^\nerror: unclosed character class"
                        .into()
                })
            );
        }

        #[test]
        fn returns_error_when_no_regex_provided() {
            let source = "Hello, World!";

            let input = [source].abi_encode();

            let result = is_match(&Bytes::from(input));
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: r#"type check failed for "offset (usize)" with data: 0000000000000000000000000000000000002000000000000000000000000000"#.into()
                })
            );
        }

        #[test]
        fn returns_error_args_are_not_strings() {
            let input = [1, 2].abi_encode();

            let result = is_match(&Bytes::from(input));
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: "ABI decoding failed: buffer overrun while deserializing".into()
                })
            );
        }
    }

    mod capture_test {
        use super::*;

        #[test]
        fn regex_capture_returns_all_captures() {
            let source = "Hello, World!";
            let regex = Regex::new(r"^(\w+), (\w+)!$").unwrap();

            let result = do_capture(source, &regex).unwrap();

            assert_eq!(result, vec![source.to_string(), "Hello".to_string(), "World".to_string()]);
        }

        #[test]
        fn first_capture_is_whole_match_even_without_captured_groups() {
            let source = "Hello World!";
            let regex = Regex::new(r"^Hello(,)? World!$").unwrap();

            let result = do_capture(source, &regex).unwrap();

            assert_eq!(result, vec![source.to_string(), "".to_string()]);
        }

        #[test]
        fn returns_error_if_no_match() {
            let source = "Hello, World!";
            let regex = Regex::new(r"^(Hello), Galaxy!$").unwrap();

            let result = do_capture(source, &regex);

            assert_eq!(result, Err("No match found"));
        }

        #[test]
        fn returns_error_when_invalid_regex() {
            let source = "Hello, World!";
            let regex = r"^[$";

            let input = [source, regex].abi_encode();

            let result = capture(&Bytes::from(input));
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: "regex parse error:\n    ^[$\n     ^\nerror: unclosed character class"
                        .into()
                })
            );
        }

        #[test]
        fn returns_error_when_no_regex_provided() {
            let source = "Hello, World!";

            let input = [source].abi_encode();

            let result = capture(&Bytes::from(input));
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: r#"type check failed for "offset (usize)" with data: 0000000000000000000000000000000000002000000000000000000000000000"#.into()
                })
            );
        }

        #[test]
        fn returns_error_args_are_not_strings() {
            let input = [1, 2].abi_encode();

            let result = capture(&Bytes::from(input));
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: "ABI decoding failed: buffer overrun while deserializing".into()
                })
            );
        }
    }

    mod validate_regex {
        use super::*;

        #[test]
        fn returns_ok_when_regex_captures_whole_string() {
            let pattern = r"^Hello, World!$";

            let result = validate_regex(pattern);

            assert_eq!(result, Ok(()));
        }

        #[test]
        fn returns_error_when_regex_misses_start_of_string_symbol() {
            let pattern = r"Hello, World!$";

            let result = validate_regex(pattern);

            assert_eq!(
                result,
                Err(r#"Regex must be surrounded by "^" and "$" pair to match the whole string"#)
            );
        }

        #[test]
        fn returns_error_when_regex_misses_end_of_string_symbol() {
            let pattern = r"^Hello, World!";

            let result = validate_regex(pattern);

            assert_eq!(
                result,
                Err(r#"Regex must be surrounded by "^" and "$" pair to match the whole string"#)
            );
        }
    }
}
