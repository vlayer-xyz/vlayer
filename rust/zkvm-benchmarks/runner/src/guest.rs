use common::GuestElf;

#[cfg(not(clippy))]
#[allow(dead_code)]
mod private {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}

#[cfg(not(clippy))]
pub static GUEST_ELF: GuestElf =
    GuestElf::new(private::RISC0_BENCHMARK_GUEST_ID, private::RISC0_BENCHMARK_GUEST_ELF);
#[cfg(clippy)]
pub static GUEST_ELF: GuestElf = GuestElf::default();
