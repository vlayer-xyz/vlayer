use std::sync::Arc;

use jmespath::{JmespathError, Variable, create_default_runtime};
use serde_json::Value;

pub fn get_value_by_path(value: &Value, path: &str) -> Result<Variable, JmespathError> {
    let runtime = create_default_runtime();
    let expression = runtime.compile(path)?;
    let value = expression.search(value)?;
    let value = Arc::try_unwrap(value).expect("Failed to unwrap value");
    Ok(value)
}

#[cfg(test)]
mod tests {

    use lazy_static::lazy_static;
    use serde_json::json;

    use super::*;

    lazy_static! {
        static ref JSON: Value = json!({
            "root": {
                "nested_level": {
                    "field_string": "field_string_value",
                    "field_number": 12,
                    "field_boolean": true,
                    "field_array": ["val1", "val2"],
                    "field_object": {},
                    "field_array_of_objects": [
                        {"key": "val01"},
                        {"key": "val02"}
                    ],
                    "field_array_of_objects_with_numbers": [
                        {"key": 1},
                        {"key": 2}
                    ],
                    "field_array_of_booleans": [false, false, true],
                    "field_array_of_numbers": [1, 2, 3]
                }
            }
        });
    }

    #[test]
    fn success_integer() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_number");
        assert_eq!(value, Ok(Variable::Number(12.into())));
    }

    #[test]
    fn success_bool() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_boolean");
        assert_eq!(value, Ok(Variable::Bool(true)));
    }

    #[test]
    fn success_string() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_string");
        assert_eq!(value, Ok(Variable::String("field_string_value".to_string())));
    }

    #[test]
    fn success_string_in_an_array() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array[1]");
        assert_eq!(value, Ok(Variable::String("val2".to_string())));
    }

    #[test]
    fn success_string_in_an_array_of_objects() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array_of_objects[1].key");
        assert_eq!(value, Ok(Variable::String("val02".to_string())));
    }

    #[test]
    fn success_number_in_an_array_of_objects() {
        let value = get_value_by_path(
            &JSON,
            "root.nested_level.field_array_of_objects_with_numbers[0].key",
        );
        assert_eq!(value, Ok(Variable::Number(1.into())));
    }

    #[test]
    fn success_numbers_in_array() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array_of_numbers[1]");
        assert_eq!(value, Ok(Variable::Number(2.into())));
    }

    #[test]
    fn success_booleans_in_array() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array_of_booleans[2]");
        assert_eq!(value, Ok(Variable::Bool(true)));
    }

    #[test]
    fn success_number_in_top_level_array() {
        let json_array = json!([
            {"key": 1},
            {"key": 2},
            {"key": 3}
        ]);
        let value = get_value_by_path(&json_array, "[2].key");
        assert_eq!(value, Ok(Variable::Number(3.into())));
    }

    #[test]
    fn success_array_length() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array | length(@)");
        assert_eq!(value, Ok(Variable::Number(2.into())));
    }

    #[test]
    fn failure_array_length() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_number | length(@)");
        assert_eq!(
            value.unwrap_err().to_string(),
            "Runtime error: Argument 0 expects type array|object|string, given number (line 0, column 39)
root.nested_level.field_number | length(@)
                                       ^\n"
        );
    }

    #[test]
    fn fail_missing() {
        assert_eq!(get_value_by_path(&JSON, "root.nested_level.field_missing"), Ok(Variable::Null));
    }

    #[test]
    fn fail_empty_json_body() {
        let empty = json!({});
        assert_eq!(get_value_by_path(&empty, "field"), Ok(Variable::Null));
    }
}
