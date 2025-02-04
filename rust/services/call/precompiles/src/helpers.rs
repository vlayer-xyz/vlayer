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

macro_rules! generate_precompiles {
    ($(($address:literal, $func:ident, $base_cost:literal, $byte_cost:literal),)*) => {
        [
            $(
                generate_precompiles!(($address, $func, $base_cost, $byte_cost)),
            )*
        ]
    };
    (($address:literal, $func:ident, $base_cost:literal, $byte_cost:literal)) => {{
        use alloy_primitives::Bytes;
        use helpers::gas_used;
        use revm::precompile::{
            u64_to_address, Precompile, PrecompileOutput, PrecompileResult, PrecompileWithAddress,
        };
        fn run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
            let gas_used = gas_used(input.len(), gas_limit, $base_cost, $byte_cost)?;
            let bytes = $func(input)?;
            Ok(PrecompileOutput::new(gas_used, bytes))
        }
        PrecompileWithAddress(u64_to_address($address), Precompile::Standard(run))
    }};
}
