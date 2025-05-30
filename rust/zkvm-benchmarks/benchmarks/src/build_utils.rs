const B_100: usize = 100;
const KB: usize = 1024;
const TEN_KB: usize = 10 * KB;
const HUNDRED_KB: usize = 100 * KB;

const DEPTH_0: usize = 0;
const DEPTH_1: usize = 1;
const DEPTH_10: usize = 10;
const DEPTH_100: usize = 100;

fn generate_json(target_size: usize, depth: usize) -> String {
    // estimate overhead of nesting
    let mut overhead = 0;
    for lvl in 1..=depth {
        // each nesting adds {"levelN":<…>}
        overhead += format!("{{\"level{lvl}\":").len() + 1;
    }

    // build flat body under (target_size - overhead)
    let limit = target_size.saturating_sub(overhead);
    let mut body = String::with_capacity(limit);
    body.push('{');
    let mut i = 1;
    while body.len() < limit {
        let entry = format!("\"key{i}\":\"value\",");
        if body.len() + entry.len() + 1 > limit {
            break;
        }
        body.push_str(&entry);
        i += 1;
    }
    if body.ends_with(',') {
        body.pop();
    }
    body.push('}');

    // wrap in nested levels
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
            let json = generate_json(B_100, DEPTH_1);
            assert!(json.contains("level1"));
            assert!(json.contains("key1"));
        }

        mod depth {
            use super::*;

            #[test]
            fn zero_depth() {
                let json = generate_json(B_100, DEPTH_0);
                assert!(json.contains("key1"));
                assert!(!json.contains("level1"));
            }

            #[test]
            fn large_depth() {
                let json = generate_json(HUNDRED_KB, DEPTH_100);
                assert!(json.contains("level100"));
            }
        }

        mod size {
            use super::*;

            const ALLOWED_PERCENTAGE_SIZE_DIFF: f32 = 0.6;

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
                let json = generate_json(target_size, DEPTH_0);
                assert_json_size_within_allowed_range(&json, target_size);
            }

            #[test]
            fn ten_kb() {
                let target_size = TEN_KB;
                let json = generate_json(target_size, DEPTH_0);
                assert_json_size_within_allowed_range(&json, target_size);
            }
        }
    }
}
