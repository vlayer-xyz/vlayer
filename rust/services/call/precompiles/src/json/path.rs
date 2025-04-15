use serde_json::Value;

pub fn get_value_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
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
        assert_eq!(value, Some(&json!(12)));
    }

    #[test]
    fn success_bool() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_boolean");
        assert_eq!(value, Some(&json!(true)));
    }

    #[test]
    fn success_string() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_string");
        assert_eq!(value, Some(&json!("field_string_value")));
    }

    #[test]
    fn success_string_in_an_array() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array[1]");
        assert_eq!(value, Some(&json!("val2")));
    }

    #[test]
    fn success_string_in_an_array_of_objects() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array_of_objects[1].key");
        assert_eq!(value, Some(&json!("val02")));
    }

    #[test]
    fn success_number_in_an_array_of_objects() {
        let value = get_value_by_path(
            &JSON,
            "root.nested_level.field_array_of_objects_with_numbers[0].key",
        );
        assert_eq!(value, Some(&json!(1)));
    }

    #[test]
    fn success_numbers_in_array() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array_of_numbers[1]");
        assert_eq!(value, Some(&json!(2)));
    }

    #[test]
    fn success_booleans_in_array() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array_of_booleans[2]");
        assert_eq!(value, Some(&json!(true)));
    }

    #[test]
    fn success_number_in_top_level_array() {
        let json_array = json!([
            {"key": 1},
            {"key": 2},
            {"key": 3}
        ]);
        let value = get_value_by_path(&json_array, "[2].key");
        assert_eq!(value, Some(&json!(3)));
    }

    #[test]
    fn success_array_length() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_array");
        let array = value.and_then(|v| v.as_array()).unwrap();
        assert_eq!(array.len(), 2);
    }

    #[test]
    fn fail_missing() {
        let value = get_value_by_path(&JSON, "root.nested_level.field_missing");
        assert!(value.is_none());
    }

    #[test]
    fn fail_empty_json_body() {
        let empty = json!({});
        let value = get_value_by_path(&empty, "field");
        assert!(value.is_none());
    }
}
