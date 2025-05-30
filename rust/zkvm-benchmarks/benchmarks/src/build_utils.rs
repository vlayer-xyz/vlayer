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

fn build_flat_body(size_limit: usize, value: &Value) -> String {
    let mut body = String::with_capacity(size_limit);
    body.push('{');
    let mut i = 1;
    while body.len() < size_limit {
        let entry = match value {
            Value::String(s) => format!("\"key{i}\":\"{s}\","),
            Value::Integer(n) => format!("\"key{i}\":{n},"),
        };
        if body.len() + entry.len() + 1 > size_limit {
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
        fn basic() {
            let json = generate_json(B_100, DEPTH_1, &STRING_VALUE);
            assert!(json.contains("level1"));
            assert!(json.contains("key1"));
        }

        mod depth {
            use super::*;

            #[test]
            fn zero_depth() {
                let json = generate_json(B_100, DEPTH_0, &STRING_VALUE);
                assert!(json.contains("key1"));
                assert!(!json.contains("level1"));
            }

            #[test]
            fn large_depth() {
                let json = generate_json(HUNDRED_KB, DEPTH_100, &STRING_VALUE);
                assert!(json.contains("level100"));
            }
        }

        mod size {
            use super::*;

            const ALLOWED_PERCENTAGE_SIZE_DIFF: f32 = 0.6;

            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_precision_loss)]
            fn assert_json_size_within_allowed_range(json: &str, target_size: usize) {
                let max_allowed_size =
                    (target_size as f32 * (1.0 + ALLOWED_PERCENTAGE_SIZE_DIFF)) as usize;
                let min_allowed_size =
                    (target_size as f32 * (1.0 - ALLOWED_PERCENTAGE_SIZE_DIFF)) as usize;
                assert!(json.len() >= min_allowed_size && json.len() <= max_allowed_size);
            }

            #[test]
            fn hundred_bytes() {
                let target_size = B_100;
                let json = generate_json(target_size, DEPTH_0, &STRING_VALUE);
                assert_json_size_within_allowed_range(&json, target_size);
            }

            #[test]
            fn ten_kb() {
                let target_size = TEN_KB;
                let json = generate_json(target_size, DEPTH_0, &STRING_VALUE);
                assert_json_size_within_allowed_range(&json, target_size);
            }
        }
    }
}
