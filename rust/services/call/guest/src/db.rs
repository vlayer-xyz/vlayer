use revm::db::CacheDB;
use wrap_state::WrapStateDb;

pub mod state;
pub mod wrap_state;

pub type GuestDb = CacheDB<WrapStateDb>;
