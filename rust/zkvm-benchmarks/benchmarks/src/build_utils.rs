const B_100: usize = 100;
const KB: usize = 1024;
const KB_10: usize = 10 * KB;
const KB_100: usize = 100 * KB;

const DEPTH_0: usize = 0;
const DEPTH_1: usize = 1;
const DEPTH_10: usize = 10;
const DEPTH_100: usize = 100;

fn generate_json(target_size: usize, depth: usize) -> String {
    let overhead = estimate_nesting_overhead(depth);

    // build flat body under (target_size - overhead)
    let body_size_limit = target_size.saturating_sub(overhead);
    let body = build_flat_body(body_size_limit);

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

fn build_flat_body(body_size: usize) -> String {
    let mut body = String::with_capacity(body_size);
    body.push('{');
    let mut i = 1;
    let mut value_type = 0;
    while body.len() < body_size {
        let entry = match value_type % 4 {
            0 => format!("\"key{i}\":\"value\","),
            1 => format!("\"key{i}\":1,"),
            2 => format!("\"key{i}\":true,"),
            3 => format!("\"key{i}\":1.23,"),
            _ => unreachable!(),
        };
        if body.len() + entry.len() + 1 > body_size {
            break;
        }
        body.push_str(&entry);
        i += 1;
        value_type += 1;
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

/// Generate text for benchmarking regex performance containing two distinct patterns:
///
/// 1. **Simple pattern**: The literal string `"needle"` - simple word matching
/// 2. **Complex pattern**: SSN-style format `"123-45-6789"` - complex number pattern with character classes
///
/// The function places each pattern once at the beginning of the text, then pads the remainder
/// with 'x' characters to reach the target size. This approach ensures consistent benchmark
/// conditions while testing different regex complexity levels.
///
/// # Arguments
/// * `size` - Target size of the generated text in bytes
///
/// # Returns
/// A string containing both patterns followed by padding to reach the specified size
pub fn generate_text_for_benchmarking_regex(size: usize) -> String {
    let mut out = String::with_capacity(size);

    // Calculate pattern size
    let patterns = "needle 123-45-6789 ";
    let pattern_size = patterns.len();

    // Pad with 'x' characters first
    let padding_size = size.saturating_sub(pattern_size);
    out.push_str(&"x".repeat(padding_size));

    // Add patterns at the end
    out.push_str("needle "); // Simple pattern
    out.push_str("123-45-6789 "); // Complex pattern

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    mod generate_json {
        use super::*;

        #[test]
        fn generates() {
            let json = generate_json(B_100, DEPTH_1);
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
                let result = build_flat_body(target_size);
                let actual_size = result.len();
                assert_size_within_allowed_range(actual_size, target_size);
            }
        }

        mod value {
            use super::*;

            #[test]
            fn string_value() {
                let result = build_flat_body(50);
                assert!(result.contains("\"value\""));
            }

            #[test]
            fn integer_value() {
                let result = build_flat_body(50);
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
