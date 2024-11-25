use alloy_primitives::Bytes;
use alloy_sol_types::SolType;
use regex::Regex;
use revm::precompile::{Precompile, PrecompileErrors, PrecompileOutput, PrecompileResult};
use url::Url;
use urlpattern::UrlPatternMatchInput;
use urlpattern::UrlPatternOptions;

use crate::{gas_used, map_to_fatal};

use urlpattern::UrlPattern;
use urlpattern::UrlPatternInit;

use alloy_sol_types::{sol_data, SolValue};
pub(super) const URL_PATTERN_TEST_PRECOMPILE: Precompile = Precompile::Standard(url_pattern_test_run);

const BASE_COST: u64 = 10;
const PER_WORD_COST: u64 = 1;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

fn url_pattern_test_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), BASE_COST, PER_WORD_COST, gas_limit)?;

    let (source, url_pattern) = decode_url_pattern_args(input)?;
    let pattern =
        <UrlPattern>::parse(url_pattern, UrlPatternOptions::default()).map_err(map_to_fatal)?;

    let parsed_url = Url::parse(&source).map_err(map_to_fatal)?;
    let result = pattern
        .test(UrlPatternMatchInput::Url(parsed_url))
        .map_err(map_to_fatal)?;

    Ok(PrecompileOutput::new(gas_used, result.abi_encode().into()))
}

fn decode_url_pattern_args(input: &Bytes) -> Result<(String, UrlPatternInit), PrecompileErrors> {
    let [source, pattern] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;
    let init: UrlPatternInit =
        UrlPatternInit::parse_constructor_string::<Regex>(pattern.as_str(), None)
            .map_err(map_to_fatal)?;
    Ok((source, init))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_url_pattern_match() {
        let source = "https://example.com/path";
        let pattern = "https://example.com/*";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(result);
    }

    #[test]
    fn test_url_pattern_no_match() {
        let source = "https://example.com/path";
        let pattern = "https://other.com/*";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(!result);
    }

    #[test]
    fn test_url_pattern_with_query() {
        let source = "https://example.com/path?key=value";
        let pattern = "https://example.com/*?*";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(result);
    }

    #[test]
    fn test_url_pattern_with_regex_pathname() {
        let pattern = "https://example.com/foo/([^\\/]+?)";
        let source = "https://example.com/foo/bar";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(result);
    }

    #[test]
    fn test_url_pattern_with_regex_for_query_params() {
        let pattern = "https://example.com/*/([^\\/]+?)";
        let source = "https://example.com/foo/bar";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(result);
    }

    #[test]
    fn test_url_pattern_with_multiple_query_params_and_wildcard() {
        let source = "https://example.com/path/test?key1=value1&key2=value2";
        let pattern = "https://example.com/*/test?key1=value1&key2=value2";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(result);
    }
    #[test]
    fn test_url_pattern_with_regex_for_query_params_match() {
        let source = "https://example.com/path/test?key1=value1&key2=value22";
        let pattern = "https://example.com/path/test?(.*key2=value\\d+.*)";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(result);
    }

    #[test]
    fn test_url_pattern_query_no_match() {
        let source = "https://example.com/path?key=wrong";
        let pattern = "https://example.com/*?key=value";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(!result);
    }

    #[test]
    fn test_url_pattern_with_fragment() {
        let source = "https://example.com/path#section";
        let pattern = "https://example.com/*#*";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(result);
    }

    #[test]
    fn test_invalid_url_pattern() {
        let source = "https://example.com/path";
        let pattern = "[invalid pattern]";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_protocol_alternative() {
        let source = "http://example.com/path";
        let pattern = "(http|https)://example.com/path";

        let input = [source, pattern].abi_encode();

        let result = url_pattern_test_run(&Bytes::from(input), 1000).unwrap();
        let result = bool::abi_decode(&result.bytes, true).unwrap();

        assert!(result);
    }
}
