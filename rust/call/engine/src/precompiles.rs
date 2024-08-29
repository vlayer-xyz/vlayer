mod verify_and_parse;

use revm::{
    precompile::{
        PrecompileError::Other,
        PrecompileErrors::Error,
        PrecompileWithAddress,
    },
    primitives::PrecompileErrors,
};
use verify_and_parse::VERIFY_AND_PARSE;

pub(crate) const VLAYER_PRECOMPILES: [PrecompileWithAddress; 1] = [VERIFY_AND_PARSE];

fn map_to_other<E: ToString>(err: E) -> PrecompileErrors {
    Error(Other(err.to_string()))
}
