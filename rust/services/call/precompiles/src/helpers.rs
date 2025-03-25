use revm::precompile::PrecompileErrors;

pub(super) type Result<T> = std::result::Result<T, PrecompileErrors>;

#[allow(clippy::needless_pass_by_value)] // More convenient to use in map_err
pub(super) fn map_to_fatal<E: ToString>(err: E) -> PrecompileErrors {
    PrecompileErrors::Fatal {
        msg: err.to_string(),
    }
}

macro_rules! generate_precompile {
    ($address:literal, $func:path, $base_cost:literal, $byte_cost:literal, $category:expr) => {{
        fn run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
            let gas_used = gas_used(input.len(), gas_limit, $base_cost, $byte_cost)?;
            let bytes = $func(input)?;
            Ok(PrecompileOutput::new(gas_used, bytes))
        }
        let inner = PrecompileWithAddress(u64_to_address($address), RawPrecompile::Standard(run));
        Precompile::new(inner, $category)
    }};
}

pub(super) use generate_precompile;
