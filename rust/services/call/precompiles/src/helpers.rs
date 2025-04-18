use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use revm::precompile::PrecompileErrors;

pub(super) type Result<T> = std::result::Result<T, PrecompileErrors>;

#[allow(clippy::needless_pass_by_value)] // More convenient to use in map_err
pub(super) fn map_to_fatal<E: ToString>(err: E) -> PrecompileErrors {
    PrecompileErrors::Fatal {
        msg: err.to_string(),
    }
}

#[allow(clippy::needless_pass_by_value)] // More convenient to use in map
pub(crate) fn abi_encode(value: impl SolValue) -> Bytes {
    value.abi_encode().into()
}

// Implements precompile address generation as described in EIP-7201:
// https://eips.ethereum.org/EIPS/eip-7201
//
// The address is derived using the following logic:
//
//     keccak256(abi.encode(uint256(keccak256("namespace")) - 1)) & ~bytes32(uint256(0xff))
//
// A "namespace" in this context is a fixed string that uniquely identifies a group of precompiles.
// In this implementation, the namespace is "vlayer.precompiles".
// By hashing this namespace and embedding its fingerprint into the resulting address,
// we ensure that all precompiles defined under this macro share a common prefix,
// while the final byte (set by `$suffix`) differentiates individual precompiles.
macro_rules! generate_precompile {
    ($suffix:literal, $func:path, $base_cost:literal, $byte_cost:literal, $category:expr) => {{
        use alloy_primitives::{Address, B256, Uint, keccak256};

        fn run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
            let gas_used = gas_used(input.len(), gas_limit, $base_cost, $byte_cost)?;
            $func(input).map(|out| PrecompileOutput::new(gas_used, out))
        }

        const SUFFIX: u8 = $suffix;

        let namespace_hash = Uint::<256, 4>::from_be_bytes(keccak256(b"vlayer.precompiles").into());
        let hash = keccak256(B256::from(namespace_hash - Uint::<256, 4>::from(1)));

        let mut addr = [0_u8; 20];
        addr[..19].copy_from_slice(&hash[..19]);

        addr[19] = SUFFIX;

        Precompile::new(
            PrecompileWithAddress(Address::from(addr), RawPrecompile::Standard(run)),
            $category,
        )
    }};
}

pub(super) use generate_precompile;
