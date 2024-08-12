use call_guest_wrapper::RISC0_CALL_GUEST_ID;
use std::env;

fn main() {
    if env::var("VLAYER_RELEASE").is_err() {
        println!("cargo:rustc-env=VLAYER_RELEASE=dev");
    }

    let risc0_call_guest_id_str = format!("{:?}", RISC0_CALL_GUEST_ID);

    println!(
        "cargo:rustc-env=RISC0_CALL_GUEST_ID={}, ",
        risc0_call_guest_id_str
    );

    vergen::EmitBuilder::builder()
        .build_timestamp()
        .git_sha(true)
        .emit()
        .unwrap();
}
