use revm::{
    precompile::{
        calc_linear_cost_u32, u64_to_address,
        PrecompileError::{Other, OutOfGas},
        PrecompileErrors::Error,
        PrecompileWithAddress,
    },
    primitives::{Bytes, Precompile, PrecompileErrors, PrecompileOutput, PrecompileResult},
};
use web_proof::verifier::verify_and_parse;

pub const VLAYER_PRECOMPILES: [PrecompileWithAddress; 1] = [VERIFY_AND_PARSE];

const VERIFY_AND_PARSE: PrecompileWithAddress = PrecompileWithAddress(
    u64_to_address(0x100),
    Precompile::Standard(verify_and_parse_run),
);

// TODO: set an accurate gas cost values reflecting the operation's computational complexity.
/// The base cost of the operation.
pub const VERIFY_AND_PARSE_BASE: u64 = 10;
/// The cost per word.
pub const VERIFY_AND_PARSE_PER_WORD: u64 = 1;

fn verify_and_parse_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = calc_linear_cost_u32(
        input.len(),
        VERIFY_AND_PARSE_BASE,
        VERIFY_AND_PARSE_PER_WORD,
    );
    if gas_used > gas_limit {
        return Err(Error(OutOfGas));
    }

    let web_proof_json = std::str::from_utf8(input).map_err(map_to_other)?;
    let web_proof = serde_json::from_str(web_proof_json).map_err(map_to_other)?;
    let web = verify_and_parse(web_proof).map_err(map_to_other)?;

    Ok(PrecompileOutput::new(gas_used, web.url.into()))
}

fn map_to_other<E: ToString>(err: E) -> PrecompileErrors {
    Error(Other(err.to_string()))
}
