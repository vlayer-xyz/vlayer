mod evm_env;
mod host;
mod into_input;

use call_common::RevmDB;
use call_db::ProofDb;
pub use call_engine::Call;
pub use host::{
    BuilderError, Config, Error, Host, PreflightResult, Prover, ProvingError, ProvingInput,
    error::preflight::Error as PreflightError,
};
use revm::db::CacheDB;

pub type HostDb = CacheDB<ProofDb>;
pub type HostDbError = <HostDb as RevmDB>::Error;

#[cfg(test)]
pub mod test_harness;
