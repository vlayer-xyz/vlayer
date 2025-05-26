use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use call_precompiles::json::get_string;

use crate::Benchmark;

macro_rules! include_json {
    ($const_name:ident, $file:literal) => {
        pub const $const_name: &str =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/json/", $file));
    };
}

include_json!(JSON_100B, "100b.json");
include_json!(JSON_1KB, "1kb.json");
include_json!(JSON_10KB, "10kb.json");
include_json!(JSON_100KB, "100kb.json");
include_json!(JSON_10K_1_LVL, "10k_1_level.json");
include_json!(JSON_10K_10_LVL, "10k_10_level.json");
include_json!(JSON_10K_100_LVL, "10k_100_level.json");

lazy_static::lazy_static! {
    static ref JSON_1_LVL: String = create_nested_path(1, "key1");
    static ref JSON_10_LVL: String = create_nested_path(10, "key1");
    static ref JSON_100_LVL: String = create_nested_path(100, "key1");
}

fn benchmark(json_body: &str, path: &str) {
    let calldata: Bytes = [json_body, path].abi_encode().into();
    let _ = get_string(&calldata).expect("get_string failed");
}

fn create_nested_path(depth: usize, key_name: &str) -> String {
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
        Benchmark::new("json_get_string_100b", || benchmark(JSON_100B, "key1"), 40_000),
        Benchmark::new("json_get_string_1kb", || benchmark(JSON_1KB, "key1"), 214_000),
        Benchmark::new("json_get_string_10kb", || benchmark(JSON_10KB, "key1"), 2_632_000),
        Benchmark::new("json_get_string_100kb", || benchmark(JSON_100KB, "key1"), 31_363_000),
        Benchmark::new(
            "json_get_string_10k_1_level",
            || benchmark(JSON_10K_1_LVL, &JSON_1_LVL),
            2_716_000,
        ),
        Benchmark::new(
            "json_get_string_10k_10_level",
            || benchmark(JSON_10K_10_LVL, &JSON_10_LVL),
            3_016_000,
        ),
        Benchmark::new(
            "json_get_string_10k_100_level",
            || benchmark(JSON_10K_100_LVL, &JSON_100_LVL),
            6_197_000,
        ),
    ]
}
