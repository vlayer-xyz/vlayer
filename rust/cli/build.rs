use call_guest_wrapper::RISC0_CALL_GUEST_ID;
use std::env;

fn main() {
    if env::var("VLAYER_RELEASE").is_err() {
        println!("cargo:rustc-env=VLAYER_RELEASE=dev");
    }

    let risc0_call_guest_id_hex: Vec<String> = RISC0_CALL_GUEST_ID
        .iter()
        .map(|&num| format!("{:08x}", num))
        .collect();

    let risc0_call_guest_id_str = format!("0x{}", risc0_call_guest_id_hex.join(""));

    println!(
        "cargo:rustc-env=RISC0_CALL_GUEST_ID={}",
        risc0_call_guest_id_str
    );

    vergen::EmitBuilder::builder()
        .build_timestamp()
        .git_sha(true)
        .emit()
        .unwrap();
}
