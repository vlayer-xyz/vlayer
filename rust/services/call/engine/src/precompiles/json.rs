use std::convert::Into;

use alloy_primitives::Bytes;
use alloy_sol_types::{sol_data, SolType, SolValue};
use revm::precompile::{Precompile, PrecompileErrors, PrecompileOutput, PrecompileResult};
use serde_json::Value;

use crate::precompiles::{gas_used, map_to_fatal};

pub(super) const JSON_GET_STRING_PRECOMPILE: Precompile = Precompile::Standard(json_get_string_run);
pub(super) const JSON_GET_INT_PRECOMPILE: Precompile = Precompile::Standard(json_get_int_run);
pub(super) const JSON_GET_BOOL_PRECOMPILE: Precompile = Precompile::Standard(json_get_bool_run);

/// The base cost of the operation.
const BASE_COST: u64 = 10;
/// The cost per word.
const PER_WORD_COST: u64 = 1;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

fn json_get_string_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let (value_by_path, json_path, gas_used) = process_input(input, gas_limit)?;
    match value_by_path {
        Value::String(result) => Ok(PrecompileOutput::new(gas_used, result.abi_encode().into())),
        _ => Err(map_to_fatal(format!(
            "Expected type 'String' at {json_path}, but found {value_by_path:?}"
        ))),
    }
}

fn json_get_int_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let (value_by_path, json_path, gas_used) = process_input(input, gas_limit)?;
    match value_by_path {
        Value::Number(num) if num.is_i64() => {
            let result = num.as_i64().unwrap();
            Ok(PrecompileOutput::new(gas_used, result.abi_encode().into()))
        }
        _ => Err(map_to_fatal(format!(
            "Expected type 'Number' at {json_path}, but found {value_by_path:?}"
        ))),
    }
}

fn json_get_bool_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let (value_by_path, json_path, gas_used) = process_input(input, gas_limit)?;
    match value_by_path {
        Value::Bool(result) => Ok(PrecompileOutput::new(gas_used, result.abi_encode().into())),
        _ => Err(map_to_fatal(format!(
            "Expected type 'Bool' at {json_path}, but found {value_by_path:?}"
        ))),
    }
}

fn process_input(input: &Bytes, gas_limit: u64) -> Result<(Value, String, u64), PrecompileErrors> {
    let gas_used = gas_used(input.len(), BASE_COST, PER_WORD_COST, gas_limit)?;
    let [body, json_path] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;
    let body = serde_json::from_str(body.as_str())
        .map_err(|err| map_to_fatal(format!("Error converting string body to json: {}", err)))?;
    let value_by_path = get_value_by_path(&body, json_path.as_str())
        .ok_or(map_to_fatal(format!("Missing value at path {json_path}")))?;
    Ok((value_by_path.clone(), json_path, gas_used))
}

fn get_value_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(value, |acc, key| {
        if let Some((key, index)) = key.split_once('[').and_then(|(k, rest)| {
            rest.strip_suffix(']')
                .and_then(|i| i.parse::<usize>().ok().map(|i| (k, i)))
        }) {
            if key.is_empty() {
                acc.get(index)
            } else {
                acc.get(key)?.get(index)
            }
        } else {
            acc.get(key)
        }
    })
}

#[cfg(test)]
mod tests {
    use revm::precompile::{
        PrecompileError::OutOfGas,
        PrecompileErrors::{Error, Fatal},
    };

    use super::*;

    const TEST_JSON: &str = r#"
            {
                "root": {
                    "nested_level": {
                        "field_string": "field_string_value",
                        "field_number": 12,
                        "field_boolean": true,
                        "field_array": ["val1", "val2"],
                        "field_object": {},
                        "field_array_of_objects": [{"key": "val01"},{"key": "val02"}]
                    }
                }
            }
            "#;
    #[test]
    fn success_integer() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

        let precompile_output =
            json_get_int_run(&abi_encoded_body_and_json_path.into(), u64::MAX).unwrap();

        let result =
            sol_data::Int::<256>::abi_decode(precompile_output.bytes.as_ref(), false).unwrap();
        let parsed: alloy_primitives::I256 = "12".parse().unwrap();

        assert_eq!(parsed, result);
    }

    #[test]
    fn success_bool() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_boolean"]);

        let precompile_output =
            json_get_bool_run(&abi_encoded_body_and_json_path.into(), u64::MAX).unwrap();

        let result = sol_data::Bool::abi_decode(precompile_output.bytes.as_ref(), false).unwrap();

        assert!(result);
    }

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
    fn success_string_in_an_array() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_array[1]"]);

        let precompile_output =
            json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX).unwrap();

        assert_eq!(
            "val2",
            sol_data::String::abi_decode(precompile_output.bytes.as_ref(), true).unwrap()
        );
    }

    #[test]
    fn success_string_in_an_array_of_objects() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_array_of_objects[1].key"]);

        let precompile_output =
            json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX).unwrap();

        assert_eq!(
            "val02",
            sol_data::String::abi_decode(precompile_output.bytes.as_ref(), true).unwrap()
        );
    }

    #[test]
    fn fail_out_of_gas() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

        let insufficient_gas_limit = 1;

        let precompile_output =
            process_input(&abi_encoded_body_and_json_path.into(), insufficient_gas_limit);

        assert!(matches!(precompile_output, Err(Error(OutOfGas))));
    }

    #[test]
    fn fail_missing() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_missing"]);

        assert!(matches!(process_input(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Missing value at path root.nested_level.field_missing"));
    }

    mod string_tests {
        use super::*;
        #[test]
        fn fail_string_number() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

            assert!(
                matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
                Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_number, but found Number(12)")
            );
        }

        #[test]
        fn fail_string_bool() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_boolean"]);

            assert!(
                matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
                Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_boolean, but found Bool(true)")
            );
        }

        #[test]
        fn fail_string_object() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

            assert!(
                matches!(json_get_string_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
                Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_object, but found Object {}")
            );
        }
    }

    mod number_tests {
        use super::*;
        #[test]
        fn fail_number_bool() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_boolean"]);

            assert!(matches!(json_get_int_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
                Err(Fatal { msg: message }) if message == "Expected type 'Number' at root.nested_level.field_boolean, but found Bool(true)"));
        }

        #[test]
        fn fail_number_string() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

            assert!(matches!(json_get_int_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
                Err(Fatal { msg: message }) if message == "Expected type 'Number' at root.nested_level.field_string, but found String(\"field_string_value\")"));
        }

        #[test]
        fn fail_number_object() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

            assert!(matches!(json_get_int_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
                Err(Fatal { msg: message }) if message == "Expected type 'Number' at root.nested_level.field_object, but found Object {}"));
        }
    }

    mod bool_tests {
        use super::*;
        #[test]
        fn fail_boolean_string() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

            assert!(matches!(json_get_bool_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
                Err(Fatal { msg: message }) if message == "Expected type 'Bool' at root.nested_level.field_string, but found String(\"field_string_value\")"));
        }
        #[test]
        fn fail_bool_number() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

            assert!(matches!(json_get_bool_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
                Err(Fatal { msg: message }) if message == "Expected type 'Bool' at root.nested_level.field_number, but found Number(12)"));
        }

        #[test]
        fn fail_bool_object() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

            assert!(matches!(json_get_bool_run(&abi_encoded_body_and_json_path.into(), u64::MAX),
                Err(Fatal { msg: message }) if message == "Expected type 'Bool' at root.nested_level.field_object, but found Object {}"));
        }
    }

    #[test]
    fn fail_empty_json_string() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["", "field"]);

        assert!(matches!(process_input(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Error converting string body to json: EOF while parsing a value at line 1 column 0"));
    }

    #[test]
    fn fail_empty_json_body() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["{}", "field"]);

        assert!(matches!(process_input(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Missing value at path field"))
    }

    #[test]
    fn fail_string_as_json() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["a string", "field"]);

        assert!(matches!(process_input(&abi_encoded_body_and_json_path.into(), u64::MAX),
            Err(Fatal { msg: message }) if message == "Error converting string body to json: expected value at line 1 column 1"))
    }
}
