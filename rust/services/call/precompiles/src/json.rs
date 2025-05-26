use alloy_primitives::Bytes;
use alloy_sol_types::sol_data;
use args::decode_args;
pub use get_float_as_int::get_float_as_int;
use jmespath::Variable;
use path::get_value_by_path;

use crate::helpers::{Result, abi_encode, map_to_fatal};

mod args;
mod get_float_as_int;
mod path;

type InputType = sol_data::FixedArray<sol_data::String, 2>;

pub fn get_string(input: &Bytes) -> Result<Bytes> {
    let (value, path) = get_value(input)?;
    value
        .as_string()
        .map(abi_encode)
        .ok_or(map_to_fatal(format!("Expected type 'String' at {path}, but found {value:?}")))
}

#[allow(clippy::unwrap_used)]
pub fn get_int(input: &Bytes) -> Result<Bytes> {
    let (value, path) = get_value(input)?;
    let numeric_value = match value {
        Variable::Number(ref num) => num.as_i64(),
        _ => None,
    };
    numeric_value
        .map(abi_encode)
        .ok_or(map_to_fatal(format!("Expected type 'Number' at {path}, but found {value:?}")))
}

pub fn get_bool(input: &Bytes) -> Result<Bytes> {
    let (value, path) = get_value(input)?;
    value
        .as_boolean()
        .map(abi_encode)
        .ok_or(map_to_fatal(format!("Expected type 'Bool' at {path}, but found {value:?}")))
}

fn get_value(input: &Bytes) -> Result<(Variable, String)> {
    let (body, path) = decode_args(input)?;
    let value_by_path = get_value_by_path(&body, path.as_str()).map_err(map_to_fatal)?;
    Ok((value_by_path, path))
}

#[cfg(test)]
mod tests {
    use alloy_sol_types::SolType;
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
            abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

        let precompile_output = get_int(&abi_encoded_body_and_json_path).unwrap();

        let result = sol_data::Int::<256>::abi_decode(precompile_output.as_ref(), false).unwrap();
        let parsed: alloy_primitives::I256 = "12".parse().unwrap();

        assert_eq!(parsed, result);
    }

    #[test]
    fn fail_missing() {
        let abi_encoded_body_and_json_path =
            abi_encode(&[TEST_JSON, "root.nested_level.field_missing"]);

        let result = get_value(&abi_encoded_body_and_json_path);
        assert_eq!(result, Ok((Variable::Null, "root.nested_level.field_missing".to_string())));
    }

    mod string_tests {
        use super::*;

        #[test]
        fn fail_string_number() {
            let abi_encoded_body_and_json_path =
                abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

            let result = get_string(&abi_encoded_body_and_json_path);
            assert_eq!(
                result,
                Err(Fatal {
                    msg: "Expected type 'String' at root.nested_level.field_number, but found Number(Number(12))".into()
                })
            );
        }

        #[test]
        fn fail_string_bool() {
            let abi_encoded_body_and_json_path =
                abi_encode(&[TEST_JSON, "root.nested_level.field_boolean"]);

            let result = get_string(&abi_encoded_body_and_json_path);
            assert_eq!(
                result,
                Err(Fatal {
                    msg: "Expected type 'String' at root.nested_level.field_boolean, but found Bool(true)".into()
                })
            );
        }

        #[test]
        fn fail_string_object() {
            let abi_encoded_body_and_json_path =
                abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

            let result = get_string(&abi_encoded_body_and_json_path);
            assert_eq!(
                result,
                Err(Fatal {
                    msg: "Expected type 'String' at root.nested_level.field_object, but found Object({})".into()
                })
            );
        }
    }

    mod number_tests {
        use super::*;

        #[test]
        fn fail_number_bool() {
            let abi_encoded_body_and_json_path =
                abi_encode(&[TEST_JSON, "root.nested_level.field_boolean"]);

            let result = get_int(&abi_encoded_body_and_json_path);
            assert_eq!(
                result,
                Err(Fatal {
                    msg: "Expected type 'Number' at root.nested_level.field_boolean, but found Bool(true)".into()
                })
            );
        }

        #[test]
        fn fail_number_string() {
            let abi_encoded_body_and_json_path =
                abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

            let result = get_int(&abi_encoded_body_and_json_path);
            assert_eq!(
                result,
                Err(Fatal {
                    msg: "Expected type 'Number' at root.nested_level.field_string, but found String(\"field_string_value\")".into()
                })
            );
        }

        #[test]
        fn fail_number_object() {
            let abi_encoded_body_and_json_path =
                abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

            let result = get_int(&abi_encoded_body_and_json_path);
            assert_eq!(
                result,
                Err(Fatal {
                    msg: "Expected type 'Number' at root.nested_level.field_object, but found Object({})".into()
                })
            );
        }
    }

    mod bool_tests {
        use super::*;
        #[test]
        fn fail_boolean_string() {
            let abi_encoded_body_and_json_path =
                abi_encode(&[TEST_JSON, "root.nested_level.field_string"]);

            let result = get_bool(&abi_encoded_body_and_json_path);
            assert_eq!(
                result,
                Err(Fatal {
                    msg: "Expected type 'Bool' at root.nested_level.field_string, but found String(\"field_string_value\")".into()
                })
            );
        }

        #[test]
        fn fail_bool_number() {
            let abi_encoded_body_and_json_path =
                abi_encode(&[TEST_JSON, "root.nested_level.field_number"]);

            let result = get_bool(&abi_encoded_body_and_json_path);
            assert_eq!(
                result,
                Err(Fatal {
                    msg: "Expected type 'Bool' at root.nested_level.field_number, but found Number(Number(12))".into()
                })
            );
        }

        #[test]
        fn fail_bool_object() {
            let abi_encoded_body_and_json_path =
                abi_encode(&[TEST_JSON, "root.nested_level.field_object"]);

            let result = get_bool(&abi_encoded_body_and_json_path);
            assert_eq!(
                result,
                Err(Fatal {
                    msg: "Expected type 'Bool' at root.nested_level.field_object, but found Object({})".into()
                })
            );
        }
    }

    #[test]
    fn fail_empty_json_body() {
        let abi_encoded_body_and_json_path = abi_encode(&["{}", "field"]);

        assert_eq!(
            get_value(&abi_encoded_body_and_json_path),
            Ok((Variable::Null, "field".to_string()))
        );
    }
}
