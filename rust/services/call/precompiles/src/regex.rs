use ::regex::Regex;
use alloy_primitives::Bytes;
use alloy_sol_types::{sol_data, SolType, SolValue};
use regex::{Captures, Match};
use revm::precompile::{Precompile, PrecompileErrors, PrecompileOutput, PrecompileResult};

use crate::{gas_used, map_to_fatal};

pub(super) const MATCH: Precompile = Precompile::Standard(match_run);
pub(super) const CAPTURE: Precompile = Precompile::Standard(capture_run);

const BASE_COST: u64 = 10;
const PER_WORD_COST: u64 = 1;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

fn match_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), BASE_COST, PER_WORD_COST, gas_limit)?;

    let (source, regex) = decode_args(input)?;
    let is_match = regex.is_match(&source);

    Ok(PrecompileOutput::new(gas_used, is_match.abi_encode().into()))
}

fn capture_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), BASE_COST, PER_WORD_COST, gas_limit)?;

    let (source, regex) = decode_args(input)?;
    let captures = do_capture(&source, &regex).map_err(map_to_fatal)?;

    Ok(PrecompileOutput::new(gas_used, captures.abi_encode().into()))
}

fn do_capture(source: &str, regex: &Regex) -> Result<Vec<String>, &'static str> {
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

fn decode_args(input: &Bytes) -> Result<(String, Regex), PrecompileErrors> {
    let [source, pattern] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;
    validate_regex(&pattern).map_err(map_to_fatal)?;
    let regex = Regex::new(&pattern).map_err(map_to_fatal)?;
    Ok((source, regex))
}

fn validate_regex(pattern: &str) -> Result<(), &'static str> {
    if !(pattern.starts_with('^') && pattern.ends_with('$')) {
        return Err(r#"Regex must be surrounded by "^" and "$" pair to match the whole string"#);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    mod match_test {
        use super::*;

        #[test]
        fn test_regex_match() {
            let source = "Hello, World!";
            let regex = r"^Hello, World!$";

            let input = [source, regex].abi_encode();

            let result = match_run(&Bytes::from(input), 1000).unwrap();
            let result = bool::abi_decode(&result.bytes, true).unwrap();

            assert!(result);
        }

        #[test]
        fn test_regex_no_match() {
            let source = "Hello, World!";
            let regex = r"^Goodbye, World!$";

            let input = [source, regex].abi_encode();

            let result = match_run(&Bytes::from(input), 1000).unwrap();
            let result = bool::abi_decode(&result.bytes, true).unwrap();

            assert!(!result);
        }

        #[test]
        fn returns_error_when_invalid_regex() {
            let source = "Hello, World!";
            let regex = r"^[$";

            let input = [source, regex].abi_encode();

            let result = match_run(&Bytes::from(input), 1000);
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

            let result = match_run(&Bytes::from(input), 1000);
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

            let result = match_run(&Bytes::from(input), 1000);
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: "buffer overrun while deserializing".into()
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

            let result = capture_run(&Bytes::from(input), 1000);
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

            let result = capture_run(&Bytes::from(input), 1000);
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

            let result = capture_run(&Bytes::from(input), 1000);
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: "buffer overrun while deserializing".into()
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
