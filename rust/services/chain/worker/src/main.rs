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
    db: ChainDb<DB>,
    provider: (),
}

lazy_static! {
    static ref EMPTY_PROOF: Vec<u8> = vec![];
    static ref SOME_CHAIN_INFO: ChainInfo =
        ChainInfo::new(1..=2, B256::with_last_byte(1), EMPTY_PROOF.as_slice());
    static ref SOME_CHAIN_UPDATE: ChainUpdate = ChainUpdate::new(SOME_CHAIN_INFO.clone(), [], []);
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
    pub fn new(db: ChainDb<DB>, chain_id: ChainId) -> Self {
        Worker {
            chain_id,
            db,
            provider: (),
        }
    }

    #[allow(dead_code)]
    pub fn init(&mut self) -> Result<(), WorkerError> {
        Ok(self
            .db
            .update_chain(self.chain_id, SOME_CHAIN_UPDATE.clone())?)
    }

    #[allow(dead_code)]
    pub fn db(&self) -> &ChainDb<DB> {
        &self.db
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use chain_db::InMemoryDatabase;

    use super::*;

    #[test]
    fn db_has_chain_info_after_init() -> Result<()> {
        let mut worker = Worker::new(ChainDb::new(InMemoryDatabase::new()), 1);

        worker.init()?;

        assert_eq!(worker.db().get_chain_info(1)?, Some(SOME_CHAIN_INFO.clone()));

        Ok(())
    }
}
