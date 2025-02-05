use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use call_precompiles::url_pattern::test;

use crate::Benchmark;

fn benchmark(source: &str, pattern: &str) {
    let calldata: Bytes = [source, pattern].abi_encode().into();
    let _ = test(&calldata).expect("Test failed");
}

fn exact_match() {
    benchmark("https://example.com/", "https://example.com/")
}

fn exact_match_long_url() {
    benchmark(
        "https://example.com/very/long/path/with/many/segments/and/a/really/long/query/string\
        ?param1=value1&param2=very_long_value_2&param3=another_long_value&param4=yet_another_value\
        &param5=final_long_value&timestamp=1234567890&session=abcdef123456789&user=johndoe\
        &action=view&category=products&subcategory=electronics&item=smartphone&brand=techbrand\
        &model=latest2023&color=midnight_black&storage=256gb&condition=new",
        "https://example.com/very/long/path/with/many/segments/and/a/really/long/query/string\
        ?param1=value1&param2=very_long_value_2&param3=another_long_value&param4=yet_another_value\
        &param5=final_long_value&timestamp=1234567890&session=abcdef123456789&user=johndoe\
        &action=view&category=products&subcategory=electronics&item=smartphone&brand=techbrand\
        &model=latest2023&color=midnight_black&storage=256gb&condition=new",
    )
}

fn fragment() {
    benchmark("https://example.com/path#section", "https://example.com/*#*")
}

fn protocol_alternative() {
    benchmark("http://example.com/path", "(http|https)://example.com/path")
}

fn regex_pathname() {
    benchmark("https://example.com/foo/bar", "https://example.com/foo/([^\\/]+?)")
}

fn regex_for_query_params() {
    benchmark(
        "https://example.com/path/test?key1=value1&key2=value2",
        "https://example.com/*/test?key1=value1&key2=value2",
    )
}

fn wildcard_path_and_query_regex() {
    benchmark(
        "https://example.com/path/test?key1=value1&key2=value2",
        "https://example.com/*/test?(.*key2=value\\d+.*)",
    )
}

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new("exact_match", exact_match, 4_010_000),
        Benchmark::new("exact_match_long_url", exact_match_long_url, 4_840_000),
        Benchmark::new("fragment", fragment, 4_020_000),
        Benchmark::new("protocol_alternative", protocol_alternative, 4_526_000),
        Benchmark::new("regex_pathname", regex_pathname, 5_337_000),
        Benchmark::new("regex_for_query_params", regex_for_query_params, 4_120_000),
        Benchmark::new("wildcard_path_and_query_regex", wildcard_path_and_query_regex, 6_160_000),
    ]
}
