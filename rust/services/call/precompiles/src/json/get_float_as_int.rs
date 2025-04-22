use alloy_primitives::Bytes;
use alloy_sol_types::{SolValue, sol};
use jmespath::Variable;
use serde_json::Value;

use crate::{
    helpers::{Result, map_to_fatal},
    json::get_value_by_path,
};

sol! {
    struct FloatInput {
        string json;
        string path;
        uint8 precision;
    }
}

use lazy_static::lazy_static;

const MAX_PRECISION: u8 = 18;
lazy_static! {
    // f64 has 52 mantissa bits, plus 1 implicit leading bit for normalized numbers
    // → gives 53 bits of precision
    // 2^53 is the first number that cannot be exactly represented
    // So we use 2^53 - 1 as the max safe integer for precise float-to-int conversion
    // Reference: https://en.wikipedia.org/wiki/Double-precision_floating-point_format#IEEE_754_double-precision_binary_floating-point_format:_binary64
    static ref MAXIMAL_PRECISE_FLOAT_VALUE: f64 = 2_f64.powi(53) - 1.0;
}

pub fn get_float_as_int(input: &Bytes) -> Result<Bytes> {
    let FloatInput {
        json,
        path,
        precision,
    } = <FloatInput as alloy_sol_types::SolType>::abi_decode(input, true).map_err(map_to_fatal)?;

    let float_val = extract_f64_from_json(&json, &path)?;

    let int_result = scale_float_to_int(float_val, precision)?;
    Ok(int_result.abi_encode().into())
}

fn extract_f64_from_json(json: &str, path: &str) -> Result<f64> {
    let json_body: Value =
        serde_json::from_str(json).map_err(|e| map_to_fatal(format!("Error parsing JSON: {e}")))?;

    let variable = get_value_by_path(&json_body, path)
        .map_err(|e| map_to_fatal(format!("Error at path {path}: {e}")))?;

    match variable {
        Variable::Number(num) => {
            let float_val = num.as_f64().ok_or_else(|| {
                map_to_fatal(format!("Number {num} at path `{path}` cannot be represented as f64"))
            })?;
            if float_val.is_nan() {
                unreachable!(
                    "NaN should not be possible: JSON cannot contain NaN values. RFC: https://datatracker.ietf.org/doc/html/rfc8259#section-6"
                );
            }
            Ok(float_val)
        }
        other => Err(map_to_fatal(format!("Expected numeric type at {path}, found {other:?}"))),
    }
}

pub fn scale_float_to_int(float_val: f64, precision: u8) -> Result<i64> {
    if precision > MAX_PRECISION {
        return Err(map_to_fatal(format!(
            "Invalid precision value: {precision}. Precision must be between 0 and {MAX_PRECISION} (inclusive)."
        )));
    }

    if float_val.abs() > *MAXIMAL_PRECISE_FLOAT_VALUE {
        return Err(map_to_fatal(format!(
            "Float value {float_val} exceeds the maximum safe value for precise conversion to i64 (limit: {}).",
            *MAXIMAL_PRECISE_FLOAT_VALUE
        )));
    };

    let power_of_ten = 10_f64.powi(precision.into());
    let scaled = float_val * power_of_ten;

    #[allow(clippy::cast_precision_loss)]
    if scaled.abs() > i64::MAX as f64 {
        return Err(map_to_fatal(format!("Scaled value {scaled} exceeds i64::MAX ({})", i64::MAX)));
    }

    #[allow(clippy::cast_possible_truncation)]
    Ok(scaled as i64)
}

#[cfg(test)]
mod tests {
    use alloy_sol_types::sol_data;
    use revm::primitives::PrecompileErrors;

    use super::*;

    mod scale_float_to_int {
        use super::*;

        mod rounding_precision {
            use super::*;

            #[test]
            fn rounds_down_when_precision_is_less_than_fraction_digits() {
                assert_eq!(scale_float_to_int(3.4, 0), Ok(3));
            }

            #[test]
            fn preserves_fraction_when_precision_matches() {
                assert_eq!(scale_float_to_int(3.4, 1), Ok(34));
            }

            #[test]
            fn pads_with_zeros_when_precision_exceeds_fraction_length() {
                assert_eq!(scale_float_to_int(3.4, 2), Ok(340));
            }
        }

        #[test]
        fn precision_too_big() {
            let result = scale_float_to_int(1.0, MAX_PRECISION + 1);
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: format!(
                        "Invalid precision value: {}. Precision must be between 0 and {MAX_PRECISION} (inclusive).",
                        MAX_PRECISION + 1
                    )
                })
            );
        }

        #[test]
        fn float_value_too_large() {
            let value = *MAXIMAL_PRECISE_FLOAT_VALUE;
            #[allow(clippy::cast_possible_truncation)]
            let value_as_int = value as i64 + 1;
            #[allow(clippy::cast_precision_loss)]
            let result = scale_float_to_int(value_as_int as f64, 0);
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: format!(
                        "Float value {} exceeds the maximum safe value for precise conversion to i64 (limit: {}).",
                        value_as_int, *MAXIMAL_PRECISE_FLOAT_VALUE
                    )
                })
            );
        }

        #[test]
        fn scaled_value_overflows_i64_max() {
            let float_val = *MAXIMAL_PRECISE_FLOAT_VALUE;
            let precision = 4;
            let result = scale_float_to_int(float_val, precision);

            let scaled = float_val * 10_f64.powi(precision.into());

            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: format!("Scaled value {} exceeds i64::MAX ({})", scaled, i64::MAX)
                })
            );
        }
    }

    mod extract_f64_from_json {
        use super::*;

        #[test]
        fn missing_value() {
            let wrong_path = "wrong_path";
            let json = r#"{"field": 1}"#;
            let result = extract_f64_from_json(json, wrong_path);
            assert_eq!(
                result,
                Err(PrecompileErrors::Fatal {
                    msg: format!("Expected numeric type at {wrong_path}, found Null")
                })
            );
        }

        #[test]
        fn invalid_json() {
            let json = r#"this is not json"#;
            let result = extract_f64_from_json(json, "field");

            let err_msg = result.unwrap_err().to_string();
            assert!(
                err_msg.contains("Error parsing JSON"),
                "Expected error message to contain 'Error parsing JSON', got: {err_msg}"
            );
        }

        #[test]
        fn success() {
            let json = r#"{"field": 1.5}"#;
            let result = extract_f64_from_json(json, "field");
            assert_eq!(result, Ok(1.5));
        }
    }

    mod get_float_as_int {
        use alloy_sol_types::SolType;

        use super::*;

        #[test]
        fn success() {
            let json = r#"{"field": 1.5}"#;

            let input = FloatInput {
                json: json.into(),
                path: "field".into(),
                precision: 1,
            };
            let encoded = input.abi_encode();

            let result_bytes = get_float_as_int(&encoded.into()).unwrap();

            let result = sol_data::Int::<256>::abi_decode(result_bytes.as_ref(), false)
                .unwrap()
                .as_i64();

            // 1.5 * 10^1 = 15
            assert_eq!(result, 15);
        }
    }
}
