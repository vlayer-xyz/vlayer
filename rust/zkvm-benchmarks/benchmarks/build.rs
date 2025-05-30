use std::{env, fs, path::Path};

use derive_new::new;

include!("src/build_utils.rs");

include!("src/build_utils.rs");

#[derive(new)]
struct JsonConfig {
    filename: &'static str,
    size_bytes: usize,
    nesting_depth: usize,
    value: &'static JsonValue,
}

#[derive(new)]
struct RegexConfig {
    filename: &'static str,
    size_bytes: usize,
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
    static ref REGEX_CONFIGS: Vec<RegexConfig> = vec![
        RegexConfig::new("100b.txt", B_100),
        RegexConfig::new("1kb.txt", KB),
        RegexConfig::new("10kb.txt", TEN_KB),
        RegexConfig::new("100kb.txt", HUNDRED_KB),
    ];
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[allow(clippy::expect_used)]
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let out = Path::new(&out_dir);

    generate_json_files(out);
    generate_regex_files(out);
}

fn write_file(out_dir: &Path, filename: &str, content: &str) {
    #[allow(clippy::panic)]
    fs::write(out_dir.join(filename), content)
        .unwrap_or_else(|e| panic!("failed to write {filename}: {e}"));
    println!("→ generated {}/{} ({} bytes)", out_dir.display(), filename, content.len());
}

fn generate_json_files(out_dir: &Path) {
    for config in JSON_CONFIGS.iter() {
        let json = generate_json(config.size_bytes, config.nesting_depth, config.value);
        write_file(out_dir, config.filename, &json);
    }
}

fn generate_regex_files(out_dir: &Path) {
    for config in REGEX_CONFIGS.iter() {
        let text = generate_text_for_benchmarking_regex(config.size_bytes);
        write_file(out_dir, config.filename, &text);
    }
}
