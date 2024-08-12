use std::env;

fn main() {
    if env::var("BUILD_TYPE").is_err() {
        println!("cargo:rustc-env=BUILD_TYPE=dev");
    }

    vergen::EmitBuilder::builder()
        .build_timestamp()
        .git_sha(true)
        .emit()
        .unwrap();
}
