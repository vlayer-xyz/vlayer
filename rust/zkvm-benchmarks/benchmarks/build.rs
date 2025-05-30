use std::{env, fs, path::Path};

use derive_new::new;

include!("src/build_utils.rs");

include!("src/build_utils.rs");

const B_100: usize = 100;
const KB: usize = 1024;
const TEN_KB: usize = 10 * KB;
const HUNDRED_KB: usize = 100 * KB;

const DEPTH_0: usize = 0;
const DEPTH_1: usize = 1;
const DEPTH_10: usize = 10;
const DEPTH_100: usize = 100;

#[derive(new)]
struct JsonConfig {
    filename: &'static str,
    size_bytes: usize,
    nesting_depth: usize,
    value: &'static Value,
}

lazy_static! {
    static ref JSON_CONFIGS: Vec<JsonConfig> = vec![
        JsonConfig::new("100b.json", B_100, DEPTH_0, &STRING_VALUE),
        JsonConfig::new("1kb.json", KB, DEPTH_0, &STRING_VALUE),
        JsonConfig::new("10kb.json", TEN_KB, DEPTH_0, &STRING_VALUE),
        JsonConfig::new("100kb.json", HUNDRED_KB, DEPTH_0, &STRING_VALUE),
        JsonConfig::new("10kb_1_level.json", TEN_KB, DEPTH_1, &STRING_VALUE),
        JsonConfig::new("10kb_10_level.json", TEN_KB, DEPTH_10, &STRING_VALUE),
        JsonConfig::new("10kb_100_level.json", TEN_KB, DEPTH_100, &STRING_VALUE),
        JsonConfig::new("10kb_with_numbers.json", TEN_KB, DEPTH_0, &INTEGER_VALUE),
    ];
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[allow(clippy::expect_used)]
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let out = Path::new(&out_dir);

    for config in JSON_CONFIGS.iter() {
        let json = generate_json(config.size_bytes, config.nesting_depth, config.value);
        #[allow(clippy::panic)]
        fs::write(out.join(config.filename), &json)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", config.filename));
        println!("→ generated {out_dir}/{} ({} bytes)", config.filename, json.len());
    }
}
