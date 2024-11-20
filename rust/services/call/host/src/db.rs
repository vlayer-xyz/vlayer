mod proof;
mod provider;
pub use proof::ProofDb;
use revm::db::CacheDB;

pub type HostDb = CacheDB<ProofDb>;
