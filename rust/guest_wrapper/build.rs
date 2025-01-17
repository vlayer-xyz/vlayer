pub fn main() {
    #[cfg(not(clippy))]
    guest_build_utils::Risc0Builder::from_env().build().unwrap();
}
