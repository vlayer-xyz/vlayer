use crate::precompiles::{gas_used, map_to_other};
use alloy_primitives::Bytes;
use alloy_sol_types::SolType;
use alloy_sol_types::{sol_data, SolValue};
use revm::precompile::{Precompile, PrecompileOutput, PrecompileResult};
use serde_json::Value;
use std::convert::Into;

pub(super) const JSON_GET_STRING_PRECOMPILE: Precompile = Precompile::Standard(json_get_string_run);

// TODO: set an accurate gas cost values reflecting the operation's computational complexity.
/// The base cost of the operation.
const BASE_COST: u64 = 10;
/// The cost per word.
const PER_WORD_COST: u64 = 1;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

fn json_get_string_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), BASE_COST, PER_WORD_COST, gas_limit)?;

    let [body, json_path] = InputType::abi_decode(input, true).map_err(map_to_other)?;
    let body = serde_json::from_str(body.as_str())
        .map_err(|err| map_to_other(format!("Error converting string body to json: {}", err)))?;

    let value_by_path = get_value_by_path(&body, json_path.as_str())
        .ok_or(map_to_other(format!("Missing value at path {json_path}")))?;

    match value_by_path {
        Value::String(result) => Ok(PrecompileOutput::new(gas_used, result.abi_encode().into())),
        _ => Err(map_to_other(format!(
            "Expected type 'String' at {json_path}, but found {value_by_path:?}"
        ))),
    }
}

fn get_value_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(value, |acc, key| acc.get(key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use revm::precompile::{
        PrecompileError::OutOfGas,
        PrecompileErrors::{Error, Fatal},
    };

    const TEST_JSON: &str = r#"
            {
                "root": {
                    "nested_level": {
                        "field_string": "field_string_value",
                        "field_number": 0,
                        "field_boolean": true,
                        "field_array": [],
                        "field_object": {}
                    }
                }
            }
            "#;

    #[test]
    fn success_string() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

        let precompile_output =
            json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX).unwrap();

        assert_eq!(
            "field_string_value",
            sol_data::String::abi_decode(precompile_output.bytes.as_ref(), true).unwrap()
        );
    }

    #[test]
    fn fail_out_of_gas() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

        let insufficient_gas_limit = 1;

        let precompile_output = json_get_string_run(
            &abi_encoded_body_and_json_path.into(),
            insufficient_gas_limit,
        );

        assert!(matches!(precompile_output, Err(Error(OutOfGas))));
    }

    #[test]
    fn fail_missing() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_missing"]);

        assert!(
            matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Missing value at path root.nested_level.field_missing")
        );
    }

    #[test]
    fn fail_number() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

        assert!(
            matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_number, but found Number(0)")
        );
    }

    #[test]
    fn fail_boolean() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_boolean"]);

        assert!(
            matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_boolean, but found Bool(true)")
        );
    }

    #[test]
    fn fail_array() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_array"]);

        assert!(
            matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_array, but found Array []")
        );
    }

    #[test]
    fn fail_object() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

        assert!(
            matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_object, but found Object {}")
        );
    }

    #[test]
    fn fail_empty_json_string() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["", "field"]);

        assert!(
            matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Error converting string body to json: EOF while parsing a value at line 1 column 0")
        );
    }

    #[test]
    fn fail_empty_json_body() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["{}", "field"]);

        assert!(
            matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Missing value at path field")
        )
    }

    #[test]
    fn fail_string_as_json() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["a string", "field"]);

        assert!(
            matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Error converting string body to json: expected value at line 1 column 1")
        )
    }
}
