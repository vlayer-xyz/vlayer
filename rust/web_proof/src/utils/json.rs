use serde_json::Value;

use crate::{errors::ParsingError, redaction::RedactedTranscriptNameValue};

pub(crate) fn json_to_redacted_transcript(
    json_str: &str,
) -> Result<Vec<RedactedTranscriptNameValue>, ParsingError> {
    let parsed_json: Value = serde_json::from_str(json_str)?;
    let mut result = Vec::new();
    flatten_json(&parsed_json, String::new(), &mut result);
    Ok(result)
}

fn flatten_json(json: &Value, prefix: String, result: &mut Vec<RedactedTranscriptNameValue>) {
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten_json(value, new_prefix, result);
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                let new_prefix = format!("{prefix}.{index}");
                flatten_json(value, new_prefix, result);
            }
        }
        Value::String(s) => {
            result.push(RedactedTranscriptNameValue {
                name: prefix,
                value: s.as_bytes().to_vec(),
            });
        }
        _ => {
            let value_str = json.to_string();
            result.push(RedactedTranscriptNameValue {
                name: prefix,
                value: value_str.into_bytes(),
            });
        }
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
            }
        }"#;
        let result = json_to_redacted_transcript(json_str).unwrap();
        let mut result = result.iter().map(ToString::to_string).collect::<Vec<_>>();
        result.sort();

        let expected_result = [
            "array.0: 1",
            "array.1: 2",
            "array.2: 3",
            "array.3.array_string: four",
            "boolean: true",
            "number: 42",
            "object.nested_number: 99.99",
            "object.nested_string: Nested",
            "string: Hello, World!",
        ];

        assert_eq!(result, expected_result);
    }
}
