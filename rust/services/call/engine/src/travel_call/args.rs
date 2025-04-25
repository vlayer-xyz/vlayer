use alloy_primitives::hex::decode;
use once_cell::sync::Lazy;
use revm::interpreter::CallInputs;

use crate::utils::evm_call::split_calldata;

// The length of an argument in call data is 32 bytes.
const ARG_LEN: usize = 32;

pub static SET_BLOCK_SELECTOR: Lazy<Box<[u8]>> = Lazy::new(|| {
    decode("87cea3ae")
        .expect("Error decoding set_block function call")
        .into_boxed_slice()
});
pub static SET_CHAIN_SELECTOR: Lazy<Box<[u8]>> = Lazy::new(|| {
    decode("ffbc5638")
        .expect("Error decoding set_chain function call")
        .into_boxed_slice()
});

pub enum Args {
    SetBlock { block_number: u64 },
    SetChain { chain_id: u64, block_number: u64 },
}

impl Args {
    pub fn from_inputs(inputs: &CallInputs) -> Self {
        let (selector, arguments_bytes) = split_calldata(inputs);
        let arguments = arguments_bytes
            .chunks_exact(ARG_LEN)
            .map(u64_from_be_slice)
            .collect::<Vec<_>>();
        if selector == SET_BLOCK_SELECTOR.as_ref() {
            let [block_number] = arguments.try_into().expect("Invalid args for set_block");
            Args::SetBlock { block_number }
        } else if selector == SET_CHAIN_SELECTOR.as_ref() {
            let [chain_id, block_number] =
                arguments.try_into().expect("Invalid args for set_chain");
            Args::SetChain {
                chain_id,
                block_number,
            }
        } else {
            panic!("Invalid travel call selector: {selector:?}")
        }
    }
}

/// Take last 8 bytes from slice and interpret as big-endian encoded u64.
/// Will trim larger numbers to u64 range, and panic if slice is smaller than 8 bytes
/// or if discarded leading bytes are non-zero.
#[allow(clippy::missing_const_for_fn)] // Remove and add const when const Option::expect is stabilized
fn u64_from_be_slice(slice: &[u8]) -> u64 {
    if slice.len() < 8 {
        panic!("u64_from_be_slice: input slice too short, must be at least 8 bytes");
    }

    let (prefix, last8) = slice.split_at(slice.len() - 8);

    if prefix.iter().any(|&b| b != 0) {
        panic!("u64_from_be_slice: value overflows u64 — leading bytes must be zero");
    }

    u64::from_be_bytes(last8.try_into().expect("slice must be exactly 8 bytes"))
}

#[cfg(test)]
mod u64_from_be_slice {
    use alloy_primitives::U256;

    use super::*;

    #[test]
    fn success() {
        let x = u64::MAX; // To use all 8 bytes
        let slice: [u8; 32] = U256::from(x).to_be_bytes();
        let y = u64_from_be_slice(&slice);
        assert_eq!(x, y)
    }

    #[test]
    #[should_panic(expected = "invalid u64 slice")]
    fn invalid() {
        let slice = [0];
        _ = u64_from_be_slice(&slice);
    }
}
