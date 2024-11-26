use alloy_primitives::Bytes;
use alloy_sol_types::{sol_data, SolType, SolValue};
use regex::Regex;
use revm::precompile::{Precompile, PrecompileErrors, PrecompileOutput, PrecompileResult};
use url::Url;
use urlpattern::{UrlPattern, UrlPatternInit, UrlPatternMatchInput, UrlPatternOptions};

use crate::{gas_used, map_to_fatal};
pub(super) const PRECOMPILE: Precompile = Precompile::Standard(test);

const BASE_COST: u64 = 10;
const PER_WORD_COST: u64 = 1;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

fn test(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), BASE_COST, PER_WORD_COST, gas_limit)?;

    let (url_to_test, url_pattern_init) = decode_args(input)?;
    let pattern = <UrlPattern>::parse(url_pattern_init, UrlPatternOptions::default())
        .map_err(map_to_fatal)?;

    let parsed_url = Url::parse(&url_to_test).map_err(map_to_fatal)?;
    let result = pattern
        .test(UrlPatternMatchInput::Url(parsed_url))
        .map_err(map_to_fatal)?;

    Ok(PrecompileOutput::new(gas_used, result.abi_encode().into()))
}

fn decode_args(input: &Bytes) -> Result<(String, UrlPatternInit), PrecompileErrors> {
    let [url_to_test, pattern] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;
    let url_pattern_init: UrlPatternInit =
        UrlPatternInit::parse_constructor_string::<Regex>(pattern.as_str(), None)
            .map_err(map_to_fatal)?;
    Ok((url_to_test, url_pattern_init))
}

#[cfg(test)]
mod test {
    use super::*;
    fn run_test(source: &str, pattern: &str, expected: bool) {
        let input = [source, pattern].abi_encode();
        let result = test(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();
        assert_eq!(result, expected);
    }
    fn run_test_expect_err(source: &str, pattern: &str, expected_msg: &str) {
        let input = [source, pattern].abi_encode();
        let result: Result<PrecompileOutput, PrecompileErrors> = test(&Bytes::from(input), 1000);
        assert!(
            matches!(
                result,
                Err(PrecompileErrors::Fatal { ref msg }) if msg == expected_msg
            ),
            "Expected Fatal error with message '{expected_msg}' but got {result:?}",
        );
    }
    #[test]
    fn invalid_url_pattern() {
        run_test_expect_err(
            "https://example.com/path",
            "[invalid pattern]",
            "a relative input without a base URL is not valid",
        );
    }

    #[test]
    fn woildcard_path_at_the_end() {
        run_test("https://example.com/path", "https://example.com/*", true);
    }

    #[test]
    fn wrong_host() {
        run_test("https://example.com/path", "https://other.com/*", false);
    }

    #[test]
    fn wildcard_path_and_query() {
        run_test("https://example.com/path?key=value", "https://example.com/*?*", true);
    }

    #[test]
    fn regex_pathname() {
        run_test("https://example.com/foo/bar", "https://example.com/foo/([^\\/]+?)", true);
    }

    #[test]
    fn regex_for_query_params() {
        run_test("https://example.com/foo/bar", "https://example.com/*/([^\\/]+?)", true);
    }

    #[test]
    fn multiple_query_params_and_wildcard() {
        run_test(
            "https://example.com/path/test?key1=value1&key2=value2",
            "https://example.com/*/test?key1=value1&key2=value2",
            true,
        );
    }

    #[test]
    fn regex_for_query_params_match() {
        run_test(
            "https://example.com/path/test?key1=value1&key2=value22",
            "https://example.com/path/test?(.*key2=value\\d+.*)",
            true,
        );
    }

    #[test]
    fn query_no_match() {
        run_test("https://example.com/path?key=wrong", "https://example.com/*?key=value", false);
    }

    #[test]
    fn fragment() {
        run_test("https://example.com/path#section", "https://example.com/*#*", true);
    }

    #[test]
    fn protocol_alternative() {
        run_test("http://example.com/path", "(http|https)://example.com/path", true);
    }
}
