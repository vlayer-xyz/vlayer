use ::regex::Regex;
use alloy_primitives::Bytes;
use alloy_sol_types::{sol_data, SolType, SolValue};
use regex::{Captures, Match};
use revm::precompile::{Precompile, PrecompileErrors, PrecompileOutput, PrecompileResult};

use crate::precompiles::{gas_used, map_to_fatal};

pub(super) const REGEX_MATCH_PRECOMPILE: Precompile = Precompile::Standard(regex_match_run);
pub(super) const REGEX_CAPTURE_PRECOMPILE: Precompile = Precompile::Standard(regex_capture_run);

const BASE_COST: u64 = 10;
const PER_WORD_COST: u64 = 1;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

fn regex_match_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), BASE_COST, PER_WORD_COST, gas_limit)?;

    let (source, regex) = decode_regex_args(input)?;
    let is_match = regex.is_match(&source);

    Ok(PrecompileOutput::new(gas_used, is_match.abi_encode().into()))
}

fn regex_capture_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), BASE_COST, PER_WORD_COST, gas_limit)?;

    let (source, regex) = decode_regex_args(input)?;
    let captures = do_capture(&source, &regex);

    Ok(PrecompileOutput::new(gas_used, captures.abi_encode().into()))
}

fn do_capture(source: &str, regex: &Regex) -> Vec<String> {
    regex
        .captures(&source)
        .as_ref()
        .map_or_else(Vec::new, captures_to_strings)
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

fn decode_regex_args(input: &Bytes) -> Result<(String, Regex), PrecompileErrors> {
    let [source, pattern] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;
    let regex = Regex::new(&pattern).map_err(map_to_fatal)?;
    Ok((source, regex))
}

#[cfg(test)]
mod match_test {
    use super::*;

    #[test]
    fn test_regex_match() {
        let source = "Hello, World!";
        let regex = r"^Hello, World!$";

        let input = [source, regex].abi_encode();

        let result = regex_match_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(result);
    }

    #[test]
    fn test_regex_no_match() {
        let source = "Hello, World!";
        let regex = r"^Goodbye, World!$";

        let input = [source, regex].abi_encode();

        let result = regex_match_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(!result);
    }

    #[test]
    fn returns_error_when_invalid_regex() {
        let source = "Hello, World!";
        let regex = r"[";

        let input = [source, regex].abi_encode();

        let result = regex_match_run(&Bytes::from(input), 1000);
        assert_eq!(
            result,
            Err(PrecompileErrors::Fatal {
                msg: "regex parse error:\n    [\n    ^\nerror: unclosed character class".into()
            })
        );
    }

    #[test]
    fn returns_error_when_no_regex_provided() {
        let source = "Hello, World!";

        let input = [source].abi_encode();

        let result = regex_match_run(&Bytes::from(input), 1000);
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

        let result = regex_match_run(&Bytes::from(input), 1000);
        assert_eq!(
            result,
            Err(PrecompileErrors::Fatal {
                msg: "buffer overrun while deserializing".into()
            })
        );
    }
}

#[cfg(test)]
mod capture_test {
    use super::*;

    #[test]
    fn regex_capture_returns_all_captures() {
        let source = "Hello, World!";
        let regex = Regex::new(r"^(\w+), (\w+)!$").unwrap();

        let result = do_capture(source, regex);

        assert_eq!(result, vec![source.to_string(), "Hello".to_string(), "World".to_string()]);
    }

    #[test]
    fn first_capture_is_whole_match_even_without_captured_groups() {
        let source = "Hello World!";
        let regex = Regex::new(r"^Hello(,)? World!$").unwrap();

        let result = do_capture(source, regex);

        assert_eq!(result, vec![source.to_string(), "".to_string()]);
    }

    #[test]
    fn returns_empty_vector_if_no_match() {
        let source = "Hello, World!";
        let regex = Regex::new(r"^(Hello), Galaxy!$").unwrap();

        let result = do_capture(source, regex);

        assert!(result.is_empty());
    }

    #[test]
    fn returns_error_when_invalid_regex() {
        let source = "Hello, World!";
        let regex = r"[";

        let input = [source, regex].abi_encode();

        let result = regex_capture_run(&Bytes::from(input), 1000);
        assert_eq!(
            result,
            Err(PrecompileErrors::Fatal {
                msg: "regex parse error:\n    [\n    ^\nerror: unclosed character class".into()
            })
        );
    }

    #[test]
    fn returns_error_when_no_regex_provided() {
        let source = "Hello, World!";

        let input = [source].abi_encode();

        let result = regex_capture_run(&Bytes::from(input), 1000);
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

        let result = regex_capture_run(&Bytes::from(input), 1000);
        assert_eq!(
            result,
            Err(PrecompileErrors::Fatal {
                msg: "buffer overrun while deserializing".into()
            })
        );
    }
}
