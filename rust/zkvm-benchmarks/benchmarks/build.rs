use std::{env, fs, path::Path};

use derive_new::new;
use lazy_static::lazy_static;

#[derive(new)]
struct JsonConfig {
    filename: &'static str,
    size_bytes: usize,
    nesting_depth: usize,
}

lazy_static! {
    static ref JSON_CONFIGS: Vec<JsonConfig> = vec![
        JsonConfig::new("100b.json", 100, 0),
        JsonConfig::new("1kb.json", 1024, 0),
        JsonConfig::new("10kb.json", 10 * 1024, 0),
        JsonConfig::new("100kb.json", 100 * 1024, 0),
        JsonConfig::new("10k_1_level.json", 10 * 1024, 1),
        JsonConfig::new("10k_10_level.json", 10 * 1024, 10),
        JsonConfig::new("10k_100_level.json", 10 * 1024, 100),
    ];
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[allow(clippy::expect_used)]
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let out = Path::new(&out_dir);

    for config in JSON_CONFIGS.iter() {
        let json = generate_json(config.size_bytes, config.nesting_depth);
        #[allow(clippy::panic)]
        fs::write(out.join(config.filename), &json)
            .unwrap_or_else(|e| panic!("failed to write {}: {}", config.filename, e));
        println!("→ generated {out_dir}/{} ({} bytes)", config.filename, json.len());
    }
}

fn generate_json(target_size: usize, depth: usize) -> String {
    // estimate overhead of nesting
    let mut overhead = 0;
    for lvl in 0..=depth {
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
    for lvl in (0..=depth).rev() {
        result = format!("{{\"level{lvl}\":{result}}}");
    }
    result
}
