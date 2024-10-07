use ::regex::Regex;
use alloy_primitives::Bytes;
use alloy_sol_types::{sol_data, SolType, SolValue};
use revm::precompile::{Precompile, PrecompileOutput, PrecompileResult};

use crate::precompiles::{gas_used, map_to_fatal};

pub(super) const REGEX_MATCH_PRECOMPILE: Precompile = Precompile::Standard(regex_match_run);

const BASE_COST: u64 = 10;
const PER_WORD_COST: u64 = 1;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

fn regex_match_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), BASE_COST, PER_WORD_COST, gas_limit)?;

    let [source, pattern] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;

    let regex = Regex::new(&pattern).map_err(map_to_fatal)?;
    let is_match = regex.is_match(&source);

    Ok(PrecompileOutput::new(gas_used, is_match.abi_encode().into()))
}

#[cfg(test)]
mod test {
    use revm::precompile::PrecompileErrors;

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
}
