use common::GuestElf;

#[cfg(not(clippy))]
#[allow(dead_code)]
mod private {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}

include!(concat!(env!("OUT_DIR"), "/guest_id.rs"));

#[cfg(not(clippy))]
pub static CALL_GUEST_ELF: GuestElf =
    GuestElf::new(private::RISC0_CALL_GUEST_ID, private::RISC0_CALL_GUEST_ELF);

#[cfg(not(clippy))]
pub static CHAIN_GUEST_ELF: GuestElf =
    GuestElf::new(private::RISC0_CHAIN_GUEST_ID, private::RISC0_CHAIN_GUEST_ELF);

#[cfg(not(clippy))]
pub static CHAIN_GUEST_ELF_WITH_CANONICAL_ID: GuestElf =
    GuestElf::new(to_u32_array(CHAIN_GUEST_IDS[0]), private::RISC0_CHAIN_GUEST_ELF);

#[allow(unused)]
const fn to_u32_array<const INPUT_SIZE: usize, const OUTPUT_SIZE: usize>(
    bytes: [u8; INPUT_SIZE],
) -> [u32; OUTPUT_SIZE] {
    const BYTES_PER_U32: usize = 4; // (u32::BITS as usize / 8)
    assert!(
        INPUT_SIZE % BYTES_PER_U32 == 0,
        "INPUT_SIZE has to be a multiple of 4 (u32::BITS/8)"
    );
    assert!(
        INPUT_SIZE / BYTES_PER_U32 == OUTPUT_SIZE,
        "INPUT_SIZE / 4 must equal OUTPUT_SIZE"
    );

    let mut arr = [0_u32; OUTPUT_SIZE];
    let mut i = 0;
    while i < OUTPUT_SIZE {
        arr[i] = u32::from_le_bytes([
            bytes[i * BYTES_PER_U32],
            bytes[i * BYTES_PER_U32 + 1],
            bytes[i * BYTES_PER_U32 + 2],
            bytes[i * BYTES_PER_U32 + 3],
        ]);
        i += 1;
    }
    arr
}

#[cfg(clippy)]
pub static CALL_GUEST_ELF: GuestElf = GuestElf::default();

#[cfg(clippy)]
pub static CHAIN_GUEST_ELF: GuestElf = GuestElf::default();

#[cfg(clippy)]
pub static CHAIN_GUEST_ELF_WITH_CANONICAL_ID: GuestElf = GuestElf::default();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let input = [1, 0, 0, 0, 2, 0, 0, 0];
        assert_eq!(to_u32_array(input), [1, 2]);
    }

    #[test]
    #[should_panic(expected = "INPUT_SIZE has to be a multiple of 4 (u32::BITS/8)")]
    fn non_multiple_of_4() {
        let input = [0];
        to_u32_array::<1, 1>(input);
    }

    #[test]
    #[should_panic(expected = "INPUT_SIZE / 4 must equal OUTPUT_SIZE")]
    fn not_matching_output_size() {
        let input = [0, 0, 0, 0];
        to_u32_array::<4, 2>(input);
    }
}
