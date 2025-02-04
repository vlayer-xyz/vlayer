use revm::precompile::{
    calc_linear_cost_u32,
    Error::OutOfGas,
    PrecompileErrors::{self, Error},
};

pub(super) type Result<T> = std::result::Result<T, PrecompileErrors>;

#[allow(clippy::needless_pass_by_value)] // More convenient to use in map_err
pub(super) fn map_to_fatal<E: ToString>(err: E) -> PrecompileErrors {
    PrecompileErrors::Fatal {
        msg: err.to_string(),
    }
}

pub(super) fn gas_used(
    bytes: usize,
    gas_limit: u64,
    base_cost: u64,
    byte_cost: u64,
) -> Result<u64> {
    const EVM_WORD_SIZE_BYTES: u64 = 32;
    let word_cost = byte_cost * EVM_WORD_SIZE_BYTES;
    let gas_used = calc_linear_cost_u32(bytes, base_cost, word_cost);
    if gas_used > gas_limit {
        Err(Error(OutOfGas))
    } else {
        Ok(gas_used)
    }
}

macro_rules! generate_precompile {
    ($config:tt) => {{
        use alloy_primitives::Bytes;
        use helpers::gas_used;
        use revm::precompile::{
            u64_to_address, Precompile, PrecompileOutput, PrecompileResult, PrecompileWithAddress,
        };
        fn run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
            let gas_used = gas_used(input.len(), gas_limit, $config.2, $config.3)?;
            let bytes = $config.1(input)?;
            Ok(PrecompileOutput::new(gas_used, bytes))
        }
        PrecompileWithAddress(u64_to_address($config.0), Precompile::Standard(run))
    }};
}

macro_rules! generate_precompiles {
    ($($config:tt,)*) => {
        [
            $(
                generate_precompile!($config),
            )*
        ]
    };
}
