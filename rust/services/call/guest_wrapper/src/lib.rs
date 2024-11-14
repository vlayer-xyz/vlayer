use common::GuestElf;

#[cfg(not(clippy))]
#[allow(dead_code)]
mod private {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}

pub fn call_guest() -> GuestElf {
    #[cfg(not(clippy))]
    {
        GuestElf::new(private::RISC0_CALL_GUEST_ID, private::RISC0_CALL_GUEST_ELF)
    }
    #[cfg(clippy)]
    {
        GuestElf::default()
    }
}
