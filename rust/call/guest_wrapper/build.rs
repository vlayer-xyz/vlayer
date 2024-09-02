fn main() {
    #[cfg(not(clippy))]
    guest_wrapper_utils::Risc0Builder::new().build().unwrap();
}
