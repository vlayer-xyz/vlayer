pub mod proof;
pub mod provider;
use call_common::RevmDB;
use revm::db::CacheDB;

pub type HostDb = CacheDB<proof::ProofDb>;
pub type HostDbError = <HostDb as RevmDB>::Error;
