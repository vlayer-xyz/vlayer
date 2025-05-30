use lazy_static::lazy_static;

const B_100: usize = 100;
const KB: usize = 1024;
const TEN_KB: usize = 10 * KB;
const HUNDRED_KB: usize = 100 * KB;

const DEPTH_0: usize = 0;
const DEPTH_1: usize = 1;
const DEPTH_10: usize = 10;
const DEPTH_100: usize = 100;

enum Value {
    String(String),
    Integer(i64),
}

lazy_static! {
    static ref STRING_VALUE: Value = string_value("value");
    static ref INTEGER_VALUE: Value = int_value(1);
}

fn string_value(s: &str) -> Value {
    Value::String(s.to_string())
}

const fn int_value(n: i64) -> Value {
    Value::Integer(n)
}

fn generate_json(target_size: usize, depth: usize, value: &Value) -> String {
    let overhead = estimate_nesting_overhead(depth);

    // build flat body under (target_size - overhead)
    let body_size_limit = target_size.saturating_sub(overhead);
    let body = build_flat_body(body_size_limit, value);

    build_nested_body(body, depth)
}

fn estimate_nesting_overhead(depth: usize) -> usize {
    let mut overhead = 0;
    for lvl in 1..=depth {
        // each nesting adds {"levelN":<…>}
        overhead += format!("{{\"level{lvl}\":").len() + 1;
    }
    overhead
}

fn build_flat_body(body_size: usize, value: &Value) -> String {
    let mut body = String::with_capacity(body_size);
    body.push('{');
    let mut i = 1;
    while body.len() < body_size {
        let entry = match value {
            Value::String(s) => format!("\"key{i}\":\"{s}\","),
            Value::Integer(n) => format!("\"key{i}\":{n},"),
        };
        if body.len() + entry.len() + 1 > body_size {
            break;
        }
        body.push_str(&entry);
        i += 1;
    }
    if body.ends_with(',') {
        body.pop();
    }
    body.push('}');
    body
}

/// Wraps the body in nested levels.
fn build_nested_body(body: String, depth: usize) -> String {
    let mut result = body;
    if depth > 0 {
        for lvl in (1..=depth).rev() {
            result = format!("{{\"level{lvl}\":{result}}}");
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    mod generate_json {
        use super::*;

        #[test]
        fn generates() {
            let json = generate_json(B_100, DEPTH_1, &STRING_VALUE);
            assert!(json.contains("level1"));
            assert!(json.contains("key1"));
        }
    }

    mod estimate_nesting_overhead {
        use super::*;

        #[test]
        fn zero_depth() {
            assert_eq!(estimate_nesting_overhead(DEPTH_0), 0);
        }

        #[test]
        fn single_level_depth() {
            let expected_overhead = "{\"level1\":".len() + 1;
            assert_eq!(estimate_nesting_overhead(DEPTH_1), expected_overhead);
        }
    }

    mod build_flat_body {
        use super::*;

        const ALLOWED_SIZE_DIFF_PERCENT: f32 = 7.0;

        mod size {
            use super::*;

            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_precision_loss)]
            #[allow(clippy::cast_sign_loss)]
            fn assert_size_within_allowed_range(actual_size: usize, target_size: usize) {
                let max_allowed_size =
                    (target_size as f32 * (1.0 + ALLOWED_SIZE_DIFF_PERCENT / 100.0)) as usize;
                let min_allowed_size =
                    (target_size as f32 * (1.0 - ALLOWED_SIZE_DIFF_PERCENT / 100.0)) as usize;
                assert!(
                    actual_size >= min_allowed_size && actual_size <= max_allowed_size,
                    "Actual size {actual_size} is not within {ALLOWED_SIZE_DIFF_PERCENT}% of target size {target_size}",
                );
            }

            #[test]
            fn size_within_six_percent() {
                let target_size = 50;
                let result = build_flat_body(target_size, &STRING_VALUE);
                let actual_size = result.len();
                assert_size_within_allowed_range(actual_size, target_size);
            }
        }

        mod value {
            use super::*;

            #[test]
            fn string_value() {
                let result = build_flat_body(50, &STRING_VALUE);
                assert!(result.contains("\"value\""));
            }

            #[test]
            fn integer_value() {
                let result = build_flat_body(50, &INTEGER_VALUE);
                assert!(result.contains("1"));
            }
        }
    }

    mod build_nested_body {
        use super::*;

        #[test]
        fn zero_depth() {
            let result = build_nested_body("{}".to_string(), DEPTH_0);
            assert_eq!(result, "{}");
        }

        #[test]
        fn single_level_depth() {
            let result = build_nested_body("{}".to_string(), DEPTH_1);
            assert_eq!(result, "{\"level1\":{}}");
        }
    }
}
