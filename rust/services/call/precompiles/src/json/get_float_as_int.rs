use alloy_primitives::Bytes;
use alloy_sol_types::{SolType, SolValue, sol};
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
    // f64 has a 52-bit mantissa
    static ref MAXIMAL_PRECISE_FLOAT_VALUE: f64 = 2_f64.powi(53) - 1.0;
}

pub fn get_float_as_int(input: &Bytes) -> Result<Bytes> {
    let FloatInput {
        json,
        path,
        precision,
    } = <FloatInput as alloy_sol_types::SolType>::abi_decode(input, true).map_err(map_to_fatal)?;
    if precision > MAX_PRECISION {
        return Err(map_to_fatal(format!(
            "Invalid precision value: {precision}. Precision must be between 0 and 18 (inclusive)."
        )));
    }
    let json_body = serde_json::from_str::<Value>(json.as_str())
        .map_err(|e| map_to_fatal(format!("Error parsing JSON: {e}")))?;

    let value = get_value_by_path(&json_body, path.as_str())
        .ok_or(map_to_fatal(format!("Missing value at path {path}")))?;

    let Some(float_val) = value.as_f64() else {
        return Err(map_to_fatal(format!("Expected numeric type at {path}, found {value:?}")));
    };

    if float_val.is_nan() {
        unreachable!("NaN should not be possible: JSON cannot contain NaN values. RFC: https://datatracker.ietf.org/doc/html/rfc8259#section-6");
    }

    let power_of_ten = 10_f64.powi(precision.into());
    let scaled = float_val * power_of_ten;

    if scaled.abs() > *MAXIMAL_PRECISE_FLOAT_VALUE {
        return Err(map_to_fatal(format!(
            "Scaled value {scaled} exceeds the maximum safe value for precise conversion to i64 (limit: {}).",
            *MAXIMAL_PRECISE_FLOAT_VALUE
        )));
    }

    #[allow(clippy::cast_possible_truncation)]
    let as_int = scaled as i64;

    Ok(as_int.abi_encode().into())
}

mod tests {
    use std::u8::MAX;

    use alloy_sol_types::sol_data;
    use revm::primitives::PrecompileErrors;

    use super::*;

    // #[test]
    // fn testing() {
    //     dbg!(*MAXIMAL_PRECISE_FLOAT_VALUE);
    //     dbg!(*MAXIMAL_PRECISE_FLOAT_VALUE + 1.0);
    // }

    // Helper to decode output as i64
    fn decode_result(bytes: Bytes) -> i64 {
        sol_data::Int::<256>::abi_decode(bytes.as_ref(), false)
            .unwrap()
            .as_i64()
    }

    #[test]
    fn float_rounding_precision() {
        let json = r#"{ "field": 3.4 }"#;

        let input_struct = FloatInput {
            json: json.into(),
            path: "field".into(),
            precision: 0 as u8,
        };
        let input0 = input_struct.abi_encode();
        assert_eq!(decode_result(get_float_as_int(&input0.into()).unwrap()), 3);

        let input_struct = FloatInput {
            json: json.into(),
            path: "field".into(),
            precision: 1 as u8,
        };
        let input1 = input_struct.abi_encode();
        assert_eq!(decode_result(get_float_as_int(&input1.into()).unwrap()), 34);

        let input_struct = FloatInput {
            json: json.into(),
            path: "field".into(),
            precision: 2 as u8,
        };
        let input2 = input_struct.abi_encode();
        assert_eq!(decode_result(get_float_as_int(&input2.into()).unwrap()), 340);
    }

    #[test]
    fn float_negative_scaling() {
        let json = r#"{ "field": -3 }"#;

        let input_struct = FloatInput {
            json: json.into(),
            path: "field".into(),
            precision: 0 as u8,
        };
        let input = input_struct.abi_encode();
        assert_eq!(decode_result(get_float_as_int(&input.into()).unwrap()), -3);
    }

    #[test]
    fn invalid_precision() {
        let json = r#"{ "field": 1 }"#;

        let input = FloatInput {
            json: json.into(),
            path: "field".into(),
            precision: MAX_PRECISION + 1,
        };
        let encoded = input.abi_encode();

        let result = get_float_as_int(&encoded.into());
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
    fn error_invalid_json() {
        let input = FloatInput {
            json: "this is not json".into(),
            path: "field".into(),
            precision: 1,
        };
        let encoded = input.abi_encode();

        let result = get_float_as_int(&encoded.into());
        assert!(matches!(
            result,
            Err(PrecompileErrors::Fatal { msg }) if msg.contains("Error parsing JSON:")
        ));
    }

    #[test]
    fn error_non_number_type() {
        let json = r#"{ "field": "hello" }"#; // properly formed JSON
        let path = "field";

        let input = FloatInput {
            json: json.into(),
            path: path.into(),
            precision: 1,
        };
        let encoded = input.abi_encode();

        let result = get_float_as_int(&encoded.into());

        let expected_msg = r#"Expected numeric type at field, found String("hello")"#;

        assert_eq!(
            result.unwrap_err(),
            PrecompileErrors::Fatal {
                msg: expected_msg.to_string()
            }
        );
    }

    #[test]
    fn error_null_value() {
        let json = r#"{ "field": null }"#;

        let input = FloatInput {
            json: json.into(),
            path: "field".into(),
            precision: 0,
        };
        let encoded = input.abi_encode();

        let result = get_float_as_int(&encoded.into());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"Expected numeric type at field, found Null"#
        );
    } 

    // #[test]
    // fn float_negative_zero() {
    //     let json = r#"{ "field": -0 }"#;

    //     for prec in [0, 2, 4] {
    //         let input = <FloatInput as alloy_sol_types::SolType>::abi_encode(&FloatInput {
    //             json: json.into(),
    //             path: "field".into(),
    //             precision: prec as u32,
    //         });
    //         assert_eq!(decode_result(get_float_as_int(&input.into()).unwrap()), 0);
    //     }
    // }

    // #[test]
    // fn float_precision_overflow() {
    //     let json = r#"{ "field": 1 }"#;

    //     let input = <FloatInput as alloy_sol_types::SolType>::abi_encode(&FloatInput {
    //         json: json.into(),
    //         path: "field".into(),
    //         precision: 100 as u32, // 10^100
    //     });
    //     let result = get_float_as_int(&input.into());
    //     assert!(matches!(result, Err(Fatal { msg: _ })));
    // }

    // #[test]
    // fn float_large_number_overflow() {
    //     let json = r#"{ "field": 1e100 }"#;

    //     let input = <FloatInput as alloy_sol_types::SolType>::abi_encode(&FloatInput {
    //         json: json.into(),
    //         path: "field".into(),
    //         precision: 1 as u32,
    //     });
    //     let result = get_float_as_int(&input.into());
    //     assert!(matches!(result, Err(Fatal { msg: _ })));
    // }
}
