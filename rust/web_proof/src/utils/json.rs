use std::vec;

use serde_json::Value;

use crate::{errors::ParsingError, redaction::RedactedTranscriptNameValue};

pub(crate) fn json_to_redacted_transcript(
    json_str: &str,
) -> Result<Vec<RedactedTranscriptNameValue>, ParsingError> {
    let parsed_json: Value = serde_json::from_str(json_str)?;
    Ok(flatten_json(&parsed_json, "$".to_string())
        .into_iter()
        .map(Into::into)
        .collect())
}

fn flatten_json(json: &Value, prefix: String) -> Vec<(String, String)> {
    match json {
        Value::Object(obj) => {
            if obj.is_empty() {
                vec![(prefix, String::new())]
            } else {
                obj.iter()
                    .flat_map(|(key, value)| flatten_json(value, format!("{prefix}.{key}")))
                    .collect()
            }
        }

        Value::Array(arr) => {
            if arr.is_empty() {
                vec![(prefix, String::new())]
            } else {
                arr.iter()
                    .enumerate()
                    .flat_map(|(index, value)| flatten_json(value, format!("{prefix}[{index}]")))
                    .collect()
            }
        }

        Value::String(s) => vec![(prefix, s.to_string())],

        _ => vec![(prefix, json.to_string())],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success() {
        let json_str = r#"{
            "string": "Hello, World!",
            "number": 42,
            "boolean": true,
            "array": [1, 2, 3, {"array_string": "four"}],
            "object": {
                "nested_string": "Nested",
                "nested_number": 99.99
            },
            "empty_object": {},
            "empty_array": []
        }"#;
        let result = json_to_redacted_transcript(json_str).unwrap();
        let mut result = result.iter().map(ToString::to_string).collect::<Vec<_>>();
        result.sort();

        let expected_result = [
            "$.array[0]: 1",
            "$.array[1]: 2",
            "$.array[2]: 3",
            "$.array[3].array_string: four",
            "$.boolean: true",
            "$.empty_array: ",
            "$.empty_object: ",
            "$.number: 42",
            "$.object.nested_number: 99.99",
            "$.object.nested_string: Nested",
            "$.string: Hello, World!",
        ];

        assert_eq!(result, expected_result);
    }
}
