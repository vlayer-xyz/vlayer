use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use call_precompiles::json::{get_int, get_string};

use crate::Benchmark;

macro_rules! include_json {
    ($const_name:ident, $file:literal) => {
        pub const $const_name: &str =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/json/", $file));
    };
}

include_json!(JSON_100B_PATH, "100b.json");
include_json!(JSON_1KB_PATH, "1kb.json");
include_json!(JSON_10KB_PATH, "10kb.json");
include_json!(JSON_100KB_PATH, "100kb.json");
include_json!(JSON_10K_1_LVL_PATH, "10k_1_level.json");
include_json!(JSON_10K_10_LVL_PATH, "10k_10_level.json");
include_json!(JSON_10K_100_LVL_PATH, "10k_100_level.json");
include_json!(JSON_10K_1000_LVL_PATH, "10kb_with_numbers.json");

lazy_static::lazy_static! {
    static ref JSON_1_LVL_KEY: String = create_nested_key_path(1, "key1");
    static ref JSON_10_LVL_KEY: String = create_nested_key_path(10, "key1");
    static ref JSON_100_LVL_KEY: String = create_nested_key_path(100, "key1");
}

fn benchmark_get_string(json_body: &str, path: &str) {
    let calldata: Bytes = [json_body, path].abi_encode().into();
    let _ = get_string(&calldata).expect("get_string failed");
}

fn benchmark_get_int(json_body: &str, path: &str) {
    let calldata: Bytes = [json_body, path].abi_encode().into();
    let _ = get_int(&calldata).expect("get_int failed");
}

fn create_nested_key_path(depth: usize, key_name: &str) -> String {
    let mut path = String::new();
    for i in 1..=depth {
        path.push_str(&format!("level{i}"));
        if i < depth {
            path.push('.');
        }
    }
    path.push('.');
    path.push_str(key_name);
    path
}

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new(
            "json_get_string_100b",
            || benchmark_get_string(JSON_100B_PATH, "key1"),
            40_000,
        ),
        Benchmark::new(
            "json_get_string_1kb",
            || benchmark_get_string(JSON_1KB_PATH, "key1"),
            214_000,
        ),
        Benchmark::new(
            "json_get_string_10kb",
            || benchmark_get_string(JSON_10KB_PATH, "key1"),
            2_632_000,
        ),
        Benchmark::new(
            "json_get_string_100kb",
            || benchmark_get_string(JSON_100KB_PATH, "key1"),
            31_363_000,
        ),
        Benchmark::new(
            "json_get_string_10k_1_level",
            || benchmark_get_string(JSON_10K_1_LVL_PATH, &JSON_1_LVL_KEY),
            2_716_000,
        ),
        Benchmark::new(
            "json_get_string_10k_10_level",
            || benchmark_get_string(JSON_10K_10_LVL_PATH, &JSON_10_LVL_KEY),
            3_016_000,
        ),
        Benchmark::new(
            "json_get_string_10k_100_level",
            || benchmark_get_string(JSON_10K_100_LVL_PATH, &JSON_100_LVL_KEY),
            6_197_000,
        ),
        Benchmark::new(
            "json_get_int_10kb",
            || benchmark_get_int(JSON_10K_1000_LVL_PATH, "key1"),
            2_632_000,
        ),
    ]
}
