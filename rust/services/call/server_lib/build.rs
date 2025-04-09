use std::fs;

fn main() {
    let cargo_lock = fs::read_to_string("../../../../Cargo.lock") // adjust path if needed
        .expect("Could not read Cargo.lock");

    let version = cargo_lock
        .split("[[package]]")
        .find(|pkg| pkg.contains("name = \"risc0-build\""))
        .and_then(|pkg| {
            pkg.lines()
                .find(|line| line.trim_start().starts_with("version = "))
                .map(|line| {
                    line.split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .trim_matches('"')
                        .to_string()
                })
        })
        .expect("Failed to extract risc0-build version from Cargo.lock");

    println!("cargo:rustc-env=RISC0_BUILD_VERSION={}", version);
}
