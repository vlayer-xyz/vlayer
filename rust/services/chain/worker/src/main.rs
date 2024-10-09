use std::sync::{Arc, RwLock};

use alloy_primitives::{ChainId, B256};
use chain_db::{ChainDb, ChainInfo, ChainUpdate, Database};
use lazy_static::lazy_static;
use thiserror::Error;

fn main() -> anyhow::Result<()> {
    Ok(())
}

#[allow(dead_code)]
struct Worker<DB: for<'a> Database<'a>> {
    chain_id: ChainId,
    db: Arc<RwLock<ChainDb<DB>>>,
    provider: (),
}

lazy_static! {
    static ref EMPTY_PROOF: Vec<u8> = vec![];
}

#[derive(Debug, Error)]
enum WorkerError {
    #[error("ChainDB error: {0}")]
    ChainDb(#[from] chain_db::ChainDbError),
}

impl<DB> Worker<DB>
where
    DB: for<'a> Database<'a>,
{
    #[allow(dead_code)]
    pub fn new(db: Arc<RwLock<ChainDb<DB>>>, chain_id: ChainId) -> Self {
        Worker {
            chain_id,
            db,
            provider: (),
        }
    }

    #[allow(dead_code)]
    pub fn init(&mut self) -> Result<(), WorkerError> {
        let chain_info = ChainInfo::new(1..=2, B256::with_last_byte(1), EMPTY_PROOF.as_slice());
        let chain_update = ChainUpdate::new(chain_info, [], []);
        Ok(self
            .db
            .write()
            .expect("poisoned lock")
            .update_chain(self.chain_id, chain_update)?)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use chain_db::InMemoryDatabase;

    use super::*;

    fn get_chain_info(db: Arc<RwLock<ChainDb<InMemoryDatabase>>>) -> Option<ChainInfo> {
        let db = db.read().expect("poisoned lock");
        db.get_chain_info(1).expect("get_chain_info failed")
    }

    fn setup_worker() -> Worker<InMemoryDatabase> {
        let db = InMemoryDatabase::new();
        let chain_db = Arc::new(RwLock::new(ChainDb::new(db)));
        Worker::new(chain_db.clone(), 1)
    }

    #[test]
    fn db_has_chain_info_after_init() -> Result<()> {
        let mut worker = setup_worker();

        worker.init()?;

        assert_eq!(
            get_chain_info(worker.db.clone()),
            Some(ChainInfo::new(1..=2, B256::with_last_byte(1), EMPTY_PROOF.as_slice()))
        );

        Ok(())
    }
}
