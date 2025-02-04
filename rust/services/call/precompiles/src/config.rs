use alloy_primitives::Bytes;
use revm::precompile::{
    calc_linear_cost_u32, u64_to_address,
    Error::OutOfGas,
    Precompile as RawPrecompile,
    PrecompileErrors::{self, Error},
    PrecompileOutput, PrecompileResult, PrecompileWithAddress,
};

use crate::{
    json::{
        GET_ARRAY_LENGTH as JSON_GET_ARRAY_LENGTH, GET_BOOL as JSON_GET_BOOL,
        GET_INT as JSON_GET_INT, GET_STRING as JSON_GET_STRING,
    },
    regex::{CAPTURE as REGEX_CAPTURE, MATCH as REGEX_MATCH},
    url_pattern::TEST as URL_PATTERN_TEST,
    verify_and_parse::VERIFY_AND_PARSE as VERIFY_WEB,
    verify_and_parse_email::VERIFY_AND_PARSE as VERIFY_EMAIL,
};

const NUM_PRECOMPILES: usize = 1;

fn gas_used(
    bytes: usize,
    gas_limit: u64,
    base_cost: u64,
    byte_cost: u64,
) -> Result<u64, PrecompileErrors> {
    let word_cost = byte_cost * 4;
    let gas_used = calc_linear_cost_u32(bytes, base_cost, word_cost);
    if gas_used > gas_limit {
        Err(Error(OutOfGas))
    } else {
        Ok(gas_used)
    }
}

macro_rules! generate_precompile {
    ($config:tt) => {{
        fn run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
            let gas_used = gas_used(input.len(), gas_limit, $config.2, $config.3)?;
            let bytes = $config.1(input)?;
            Ok(PrecompileOutput::new(gas_used, bytes))
        }
        PrecompileWithAddress(u64_to_address($config.0), RawPrecompile::Standard(run))
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

#[rustfmt::skip]
pub const PRECOMPILES: [PrecompileWithAddress; NUM_PRECOMPILES] = generate_precompiles![
    (0x100, crate::verify_and_parse::verify_and_parse_run_2, 1000, 10),
];
