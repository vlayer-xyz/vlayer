#[allow(clippy::unwrap_used)]
pub fn main() {
    #[cfg(not(clippy))]
    guest_build_utils::Risc0Builder::from_env().build().unwrap();
    // This is cheap operation, so we can always execute it.
    guest_build_utils::chain_guest_id::generate_rust(true).unwrap();
}
