use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use call_precompiles::json::get_string;

use crate::Benchmark;

// Include JSON files as string constants
const JSON_100B: &str = include_str!("../../../assets/json/100b.json");
const JSON_1KB: &str = include_str!("../../../assets/json/1kb.json");
const JSON_10KB: &str = include_str!("../../../assets/json/10kb.json");
const JSON_100KB: &str = include_str!("../../../assets/json/100kb.json");
const JSON_10K_1_LEVEL: &str = include_str!("../../../assets/json/10k_1_level.json");
const JSON_10K_10_LEVEL: &str = include_str!("../../../assets/json/10k_10_level.json");
const JSON_10K_100_LEVEL: &str = include_str!("../../../assets/json/10k_100_level.json");

lazy_static::lazy_static! {
    static ref PATH_10_LEVELS: String = create_nested_path(10, "key1");
    static ref PATH_100_LEVELS: String = create_nested_path(100, "key1");
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
        Benchmark::new("json_get_string_100b", || benchmark(JSON_100B, "key1"), 40_738),
        Benchmark::new("json_get_string_1kb", || benchmark(JSON_1KB, "key1"), 214_246),
        Benchmark::new("json_get_string_10kb", || benchmark(JSON_10KB, "key1"), 2_632_277),
        Benchmark::new("json_get_string_100kb", || benchmark(JSON_100KB, "key1"), 31_363_990),
        Benchmark::new(
            "json_get_string_10k_1_level",
            || benchmark(JSON_10K_1_LEVEL, "level1.key1"),
            2_716_219,
        ),
        Benchmark::new(
            "json_get_string_10k_10_level",
            || benchmark(JSON_10K_10_LEVEL, &PATH_10_LEVELS),
            3_016_135,
        ),
        Benchmark::new(
            "json_get_string_10k_100_level",
            || benchmark(JSON_10K_100_LEVEL, &PATH_100_LEVELS),
            6_197_693,
        ),
    ]
}
