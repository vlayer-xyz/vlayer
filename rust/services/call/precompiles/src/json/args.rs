use alloy_primitives::Bytes;
use alloy_sol_types::SolType;
use serde_json::Value;

use super::InputType;
use crate::helpers::{Result, map_to_fatal};

pub(crate) fn decode_args(input: &Bytes) -> Result<(Value, String)> {
    let (body, path) = abi_decode_args(input)?;
    deserialize_args(&body, path)
}

fn abi_decode_args(input: &Bytes) -> Result<(String, String)> {
    let [body, path] = InputType::abi_decode(input, true).map_err(map_to_fatal)?;
    Ok((body, path))
}

fn deserialize_args(body: &str, path: String) -> Result<(Value, String)> {
    let body = serde_json::from_str(body)
        .map_err(|err| map_to_fatal(format!("Error converting string body to json: {err}")))?;
    Ok((body, path))
}

#[cfg(test)]
mod tests {
    use revm::precompile::PrecompileErrors::Fatal;

    use super::*;
    use crate::helpers::abi_encode;

    #[test]
    fn fail_empty_json_string() {
        let input = abi_encode(&["", "field"]);

        assert_eq!(decode_args(&input).unwrap_err(), Fatal { msg: "Error converting string body to json: EOF while parsing a value at line 1 column 0".to_string() });
    }

    #[test]
    fn fail_string_as_json() {
        let input = abi_encode(&["a string", "field"]);

        assert_eq!(
            decode_args(&input).unwrap_err(),
            Fatal {
                msg: "Error converting string body to json: expected value at line 1 column 1"
                    .to_string()
            }
        );
    }
}
