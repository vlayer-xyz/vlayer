use std::convert::Into;

use alloy_primitives::Bytes;
use alloy_sol_types::{SolType, SolValue, sol_data};
use path::get_value_by_path;
use serde_json::Value;

use crate::helpers::{Result, map_to_fatal};

mod path;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

#[allow(clippy::needless_pass_by_value)]
fn abi_encode(value: impl SolValue) -> Bytes {
    value.abi_encode().into()
}

pub(super) fn get_string(input: &Bytes) -> Result<Bytes> {
    let (value, path) = get_value(input)?;
    value
        .as_str()
        .map(abi_encode)
        .ok_or(map_to_fatal(format!("Expected type 'String' at {path}, but found {value:?}")))
}

#[allow(clippy::unwrap_used)]
pub(super) fn get_int(input: &Bytes) -> Result<Bytes> {
    let (value, path) = get_value(input)?;
    value
        .as_i64()
        .map(abi_encode)
        .ok_or(map_to_fatal(format!("Expected type 'Number' at {path}, but found {value:?}")))
}

pub(super) fn get_bool(input: &Bytes) -> Result<Bytes> {
    let (value, path) = get_value(input)?;
    value
        .as_bool()
        .map(abi_encode)
        .ok_or(map_to_fatal(format!("Expected type 'Bool' at {path}, but found {value:?}")))
}

pub(super) fn get_array_length(input: &Bytes) -> Result<Bytes> {
    get_array_len(input).map(|len| len.abi_encode().into())
}

fn get_value(input: &Bytes) -> Result<(Value, String)> {
    let (body, path) = decode_args(input)?;
    let value_by_path = get_value_by_path(&body, path.as_str())
        .ok_or(map_to_fatal(format!("Missing value at path {path}")))?;
    Ok((value_by_path.clone(), path))
}

fn get_array_len(input: &Bytes) -> Result<u64> {
    let (body, path) = decode_args(input)?;
    let value_by_path = get_array_length_by_path(&body, path.as_str())
        .ok_or(map_to_fatal(format!("Missing value at path {path}")))?;
    Ok(value_by_path.try_into().unwrap())
}

fn get_array_length_by_path(value: &Value, path: &str) -> Option<usize> {
    if path.is_empty() {
        value.as_array().map(std::vec::Vec::len)
    } else {
        get_value_by_path(value, path).and_then(|v| v.as_array().map(std::vec::Vec::len))
    }
}

fn decode_args(input: &Bytes) -> Result<(Value, String)> {
    let [body, path] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;
    let body = serde_json::from_str(body.as_str())
        .map_err(|err| map_to_fatal(format!("Error converting string body to json: {err}")))?;
    Ok((body, path))
}

#[cfg(test)]
mod tests {
    use revm::precompile::PrecompileErrors::Fatal;

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
                        "field_array_of_objects": [{"key": "val01"},{"key": "val02"}],
                        "field_array_of_objects_with_numbers" : [{"key": 1}, {"key": 2}],
                        "field_array_of_booleans": [false, false, true],
                        "field_array_of_numbers": [1, 2, 3]
                    }
                }
            }
            "#;

    #[test]
    fn success_integer() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

        let precompile_output = get_int(&abi_encoded_body_and_json_path.into()).unwrap();

        let result = sol_data::Int::<256>::abi_decode(precompile_output.as_ref(), false).unwrap();
        let parsed: alloy_primitives::I256 = "12".parse().unwrap();

        assert_eq!(parsed, result);
    }

    #[test]
    fn fail_missing() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_missing"]);

        assert!(matches!(get_value(&abi_encoded_body_and_json_path.into()),
            Err(Fatal { msg: message }) if message == "Missing value at path root.nested_level.field_missing"));
    }

    mod string_tests {
        use super::*;
        #[test]
        fn fail_string_number() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

            assert!(matches!(get_string(&abi_encoded_body_and_json_path.into()),
                Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_number, but found Number(12)"));
        }

        #[test]
        fn fail_string_bool() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_boolean"]);

            assert!(matches!(get_string(&abi_encoded_body_and_json_path.into()),
                Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_boolean, but found Bool(true)"));
        }

        #[test]
        fn fail_string_object() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

            assert!(matches!(get_string(&abi_encoded_body_and_json_path.into()),
                Err(Fatal { msg: message }) if message == "Expected type 'String' at root.nested_level.field_object, but found Object {}"));
        }
    }

    mod number_tests {
        use super::*;
        #[test]
        fn fail_number_bool() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_boolean"]);

            assert!(matches!(get_int(&abi_encoded_body_and_json_path.into()),
                Err(Fatal { msg: message }) if message == "Expected type 'Number' at root.nested_level.field_boolean, but found Bool(true)"));
        }

        #[test]
        fn fail_number_string() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

            assert!(matches!(get_int(&abi_encoded_body_and_json_path.into()),
                Err(Fatal { msg: message }) if message == "Expected type 'Number' at root.nested_level.field_string, but found String(\"field_string_value\")"));
        }

        #[test]
        fn fail_number_object() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

            assert!(matches!(get_int(&abi_encoded_body_and_json_path.into()),
                Err(Fatal { msg: message }) if message == "Expected type 'Number' at root.nested_level.field_object, but found Object {}"));
        }
    }

    mod bool_tests {
        use super::*;
        #[test]
        fn fail_boolean_string() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

            assert!(matches!(get_bool(&abi_encoded_body_and_json_path.into()),
                Err(Fatal { msg: message }) if message == "Expected type 'Bool' at root.nested_level.field_string, but found String(\"field_string_value\")"));
        }
        #[test]
        fn fail_bool_number() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

            assert!(matches!(get_bool(&abi_encoded_body_and_json_path.into()),
                Err(Fatal { msg: message }) if message == "Expected type 'Bool' at root.nested_level.field_number, but found Number(12)"));
        }

        #[test]
        fn fail_bool_object() {
            let abi_encoded_body_and_json_path =
                InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

            assert!(matches!(get_bool(&abi_encoded_body_and_json_path.into()),
                Err(Fatal { msg: message }) if message == "Expected type 'Bool' at root.nested_level.field_object, but found Object {}"));
        }
    }

    #[test]
    fn fail_empty_json_string() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["", "field"]);

        assert!(matches!(get_value(&abi_encoded_body_and_json_path.into()),
            Err(Fatal { msg: message }) if message == "Error converting string body to json: EOF while parsing a value at line 1 column 0"));
    }

    #[test]
    fn fail_empty_json_body() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["{}", "field"]);

        assert!(matches!(get_value(&abi_encoded_body_and_json_path.into()),
            Err(Fatal { msg: message }) if message == "Missing value at path field"))
    }

    #[test]
    fn fail_string_as_json() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["a string", "field"]);

        assert!(matches!(get_value(&abi_encoded_body_and_json_path.into()),
            Err(Fatal { msg: message }) if message == "Error converting string body to json: expected value at line 1 column 1"))
    }
}
