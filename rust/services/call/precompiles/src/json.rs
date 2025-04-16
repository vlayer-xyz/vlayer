use std::convert::Into;

use alloy_primitives::Bytes;
use alloy_sol_types::{SolType, SolValue, sol_data};
use serde_json::Value;

use crate::helpers::{Result, map_to_fatal};

type InputType = sol_data::FixedArray<sol_data::String, 2>;

pub(super) fn get_string(input: &Bytes) -> Result<Bytes> {
    let (value_by_path, json_path) = process_input(input)?;
    match value_by_path {
        Value::String(result) => Ok(result.abi_encode().into()),
        _ => Err(map_to_fatal(format!(
            "Expected type 'String' at {json_path}, but found {value_by_path:?}"
        ))),
    }
}

#[allow(clippy::unwrap_used)]
pub(super) fn get_int(input: &Bytes) -> Result<Bytes> {
    let (value_by_path, json_path) = process_input(input)?;
    match value_by_path {
        Value::Number(num) if num.is_i64() => {
            let result = num.as_i64().unwrap();
            Ok(result.abi_encode().into())
        }
        _ => Err(map_to_fatal(format!(
            "Expected type 'Number' at {json_path}, but found {value_by_path:?}"
        ))),
    }
}

pub(super) fn get_bool(input: &Bytes) -> Result<Bytes> {
    let (value_by_path, json_path) = process_input(input)?;
    match value_by_path {
        Value::Bool(result) => Ok(result.abi_encode().into()),
        _ => Err(map_to_fatal(format!(
            "Expected type 'Bool' at {json_path}, but found {value_by_path:?}"
        ))),
    }
}

pub(super) fn get_array_length(input: &Bytes) -> Result<Bytes> {
    process_input_arr(input).map(|len| len.abi_encode().into())
}

fn process_input(input: &Bytes) -> Result<(Value, String)> {
    let (body, json_path) = pre_process_input(input)?;
    let value_by_path = get_value_by_path(&body, json_path.as_str())
        .ok_or(map_to_fatal(format!("Missing value at path {json_path}")))?;
    Ok((value_by_path.clone(), json_path))
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

#[allow(clippy::unwrap_used)]
fn process_input_arr(input: &Bytes) -> Result<u64> {
    let (body, json_path) = pre_process_input(input)?;
    let value_by_path = get_array_length_by_path(&body, json_path.as_str())
        .ok_or(map_to_fatal(format!("Missing value at path {json_path}")))?;
    Ok(value_by_path.try_into().unwrap())
}

fn get_array_length_by_path(value: &Value, path: &str) -> Option<usize> {
    if path.is_empty() {
        value.as_array().map(std::vec::Vec::len)
    } else {
        get_value_by_path(value, path).and_then(|v| v.as_array().map(std::vec::Vec::len))
    }
}

fn pre_process_input(input: &Bytes) -> Result<(Value, String)> {
    let [body, json_path] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;
    let body = serde_json::from_str(body.as_str())
        .map_err(|err| map_to_fatal(format!("Error converting string body to json: {err}")))?;
    Ok((body, json_path))
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
    const TEST_JSON_ARRAY: &str = r#"
            [
                {"key": 1},
                {"key": 2},
                {"key": 3}
            ]
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
    fn success_bool() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_boolean"]);

        let precompile_output = get_bool(&abi_encoded_body_and_json_path.into()).unwrap();

        let result = sol_data::Bool::abi_decode(precompile_output.as_ref(), false).unwrap();

        assert!(result);
    }

    #[test]
    fn success_string() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

        let precompile_output = get_string(&abi_encoded_body_and_json_path.into()).unwrap();

        assert_eq!(
            "field_string_value",
            sol_data::String::abi_decode(precompile_output.as_ref(), true).unwrap()
        );
    }

    #[test]
    fn success_string_in_an_array() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_array[1]"]);

        let precompile_output = get_string(&abi_encoded_body_and_json_path.into()).unwrap();

        assert_eq!("val2", sol_data::String::abi_decode(precompile_output.as_ref(), true).unwrap());
    }

    #[test]
    fn success_string_in_an_array_of_objects() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_array_of_objects[1].key"]);

        let precompile_output = get_string(&abi_encoded_body_and_json_path.into()).unwrap();

        assert_eq!(
            "val02",
            sol_data::String::abi_decode(precompile_output.as_ref(), true).unwrap()
        );
    }

    #[test]
    fn success_number_in_an_array_of_objects() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&[
            TEST_JSON,
            "root.nested_level.field_array_of_objects_with_numbers[0].key",
        ]);

        let precompile_output = get_int(&abi_encoded_body_and_json_path.into()).unwrap();

        let result = sol_data::Int::<256>::abi_decode(precompile_output.as_ref(), false).unwrap();
        let parsed: alloy_primitives::I256 = "1".parse().unwrap();
        assert_eq!(parsed, result);
    }

    #[test]
    fn success_numbers_in_array() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_array_of_numbers[1]"]);

        let precompile_output = get_int(&abi_encoded_body_and_json_path.into()).unwrap();

        let result = sol_data::Int::<256>::abi_decode(precompile_output.as_ref(), false).unwrap();
        let parsed: alloy_primitives::I256 = "2".parse().unwrap();
        assert_eq!(parsed, result);
    }

    #[test]
    fn success_booleans_in_array() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_array_of_booleans[2]"]);

        let precompile_output = get_bool(&abi_encoded_body_and_json_path.into()).unwrap();

        let result = sol_data::Bool::abi_decode(precompile_output.as_ref(), false).unwrap();

        assert!(result);
    }

    #[test]
    fn success_number_in_top_level_array() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&[TEST_JSON_ARRAY, "[2].key"]);

        let precompile_output = get_int(&abi_encoded_body_and_json_path.into()).unwrap();

        let result = sol_data::Int::<256>::abi_decode(precompile_output.as_ref(), false).unwrap();
        let parsed: alloy_primitives::I256 = "3".parse().unwrap();
        assert_eq!(parsed, result);
    }

    #[test]
    fn success_array_length() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_array"]);

        let precompile_output = get_array_length(&abi_encoded_body_and_json_path.into()).unwrap();

        let result = sol_data::Int::<256>::abi_decode(precompile_output.as_ref(), false).unwrap();
        let parsed: alloy_primitives::I256 = "2".parse().unwrap();

        assert_eq!(parsed, result);
    }

    #[test]
    fn fail_missing() {
        let abi_encoded_body_and_json_path =
            InputType::abi_encode(&[TEST_JSON, "root.nested_level.field_missing"]);

        assert!(matches!(process_input(&abi_encoded_body_and_json_path.into()),
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

        assert!(matches!(process_input(&abi_encoded_body_and_json_path.into()),
            Err(Fatal { msg: message }) if message == "Error converting string body to json: EOF while parsing a value at line 1 column 0"));
    }

    #[test]
    fn fail_empty_json_body() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["{}", "field"]);

        assert!(matches!(process_input(&abi_encoded_body_and_json_path.into()),
            Err(Fatal { msg: message }) if message == "Missing value at path field"))
    }

    #[test]
    fn fail_string_as_json() {
        let abi_encoded_body_and_json_path = InputType::abi_encode(&["a string", "field"]);

        assert!(matches!(process_input(&abi_encoded_body_and_json_path.into()),
            Err(Fatal { msg: message }) if message == "Error converting string body to json: expected value at line 1 column 1"))
    }
}
