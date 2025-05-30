use std::{env, fs, path::Path};

use derive_new::new;

include!("src/build_utils.rs");

#[derive(new)]
struct JsonConfig {
    filename: &'static str,
    size_bytes: usize,
    nesting_depth: usize,
}

lazy_static! {
    static ref JSON_CONFIGS: Vec<JsonConfig> = vec![
        JsonConfig::new("100b.json", B_100, DEPTH_0),
        JsonConfig::new("1kb.json", KB, DEPTH_0),
        JsonConfig::new("10kb.json", TEN_KB, DEPTH_0),
        JsonConfig::new("100kb.json", HUNDRED_KB, DEPTH_0),
        JsonConfig::new("10k_1_level.json", TEN_KB, DEPTH_1),
        JsonConfig::new("10k_10_level.json", TEN_KB, DEPTH_10),
        JsonConfig::new("10k_100_level.json", TEN_KB, DEPTH_100),
    ];
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[allow(clippy::expect_used)]
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let out = Path::new(&out_dir);

    for config in JSON_CONFIGS.iter() {
        let json = generate_json(config.size_bytes, config.nesting_depth, &STRING_VALUE);
        #[allow(clippy::panic)]
        fs::write(out.join(config.filename), &json)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", config.filename));
        println!("→ generated {out_dir}/{} ({} bytes)", config.filename, json.len());
    }

    let json_with_integer_value = generate_json(TEN_KB, DEPTH_0, &INTEGER_VALUE);
    #[allow(clippy::panic)]
    fs::write(out.join("10kb_with_numbers.json"), &json_with_integer_value)
        .unwrap_or_else(|e| panic!("failed to write 10kb_with_numbers.json: {e}"));
    println!(
        "→ generated {out_dir}/10kb_with_numbers.json ({} bytes)",
        json_with_integer_value.len()
    );
}
