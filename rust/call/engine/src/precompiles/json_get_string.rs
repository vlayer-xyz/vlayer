use crate::precompiles::{gas_used, map_to_other};
use alloy_primitives::Bytes;
use alloy_sol_types::sol_data;
use alloy_sol_types::SolType;
use revm::precompile::{Precompile, PrecompileOutput, PrecompileResult};
use serde_json::Value;
use std::convert::Into;

pub(crate) const JSON_GET_STRING_PRECOMPILE: Precompile = Precompile::Standard(json_get_string_run);

// TODO: set an accurate gas cost values reflecting the operation's computational complexity.
/// The base cost of the operation.
const JSON_GET_STRING_BASE: u64 = 10;
/// The cost per word.
const JSON_GET_STRING_PER_WORD: u64 = 1;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

fn json_get_string_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let [body, json_path]: [String; 2] =
        InputType::abi_decode(input, true).map_err(map_to_other)?;

    let gas_used = gas_used(
        input.len(),
        JSON_GET_STRING_BASE,
        JSON_GET_STRING_PER_WORD,
        gas_limit,
    )?;

    let v: Value = serde_json::from_str(body.as_str()).map_err(map_to_other)?;

    match get_value_by_path(&v, json_path.as_str()).ok_or(map_to_other("Missing value at paths"))? {
        Value::String(result) => Ok(PrecompileOutput::new(gas_used, result.to_string().into())),
        _ => Err(map_to_other("Not a string at path")),
    }
}

fn get_value_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(value, |acc, key| acc.get(key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let body = r#"
            {
                "root": {
                    "level1": {
                        "level2": "level2_string_value"
                    }
                }
            }
            "#;
        let json_path = "root.level1.level2";

        let abi_encoded_body_and_json_path = InputType::abi_encode(&[body, json_path]);

        let precompile_output =
            json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX).unwrap();
        let precompile_result = std::str::from_utf8(precompile_output.bytes.as_ref()).unwrap();

        assert_eq!("level2_string_value", precompile_result);
    }
}
