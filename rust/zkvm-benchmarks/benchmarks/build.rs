use std::{env, fs, path::Path};

use derive_new::new;

include!("src/build_utils.rs");

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
