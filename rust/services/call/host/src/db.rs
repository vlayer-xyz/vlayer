pub mod proof;
mod provider;
use revm::db::CacheDB;

pub type HostDb = CacheDB<proof::ProofDb>;
