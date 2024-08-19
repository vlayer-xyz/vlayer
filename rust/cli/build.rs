use std::env;

fn main() {
    if env::var("VLAYER_RELEASE").is_err() {
        println!("cargo:rustc-env=VLAYER_RELEASE=dev");
    }

    vergen::EmitBuilder::builder()
        .build_date()
        .git_sha(true)
        .emit()
        .unwrap();
}
