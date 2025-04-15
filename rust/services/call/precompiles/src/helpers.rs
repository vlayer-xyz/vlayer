use revm::precompile::PrecompileErrors;

pub(super) type Result<T> = std::result::Result<T, PrecompileErrors>;

#[allow(clippy::needless_pass_by_value)] // More convenient to use in map_err
pub(super) fn map_to_fatal<E: ToString>(err: E) -> PrecompileErrors {
    PrecompileErrors::Fatal {
        msg: err.to_string(),
    }
}

macro_rules! generate_precompile {
    ($suffix:literal, $func:path, $base_cost:literal, $byte_cost:literal, $category:expr) => {{
        use alloy_primitives::{Address, keccak256};

        fn run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
            let gas_used = gas_used(input.len(), gas_limit, $base_cost, $byte_cost)?;
            $func(input).map(|out| PrecompileOutput::new(gas_used, out))
        }

        let mut addr = [0_u8; 20];
        addr[..18].copy_from_slice(&keccak256(b"vlayer.precompiles")[..18]);
        addr[18..].copy_from_slice(&(($suffix as u16).to_be_bytes()));

        Precompile::new(
            PrecompileWithAddress(Address::from(addr), RawPrecompile::Standard(run)),
            $category,
        )
    }};
}

pub(super) use generate_precompile;
