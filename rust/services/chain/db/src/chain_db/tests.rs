use crate::in_memory::InMemoryDatabase;
use anyhow::Result;

use super::*;

fn get_test_db() -> ChainDb<InMemoryDatabase> {
    let db = InMemoryDatabase::new();
    ChainDb::new(db)
}

#[test]
fn chain_info_get_insert() -> Result<()> {
    let mut db = get_test_db();
    let chain_id = 1;
    let chain_info = ChainInfo {
        first_block: 0,
        last_block: 1,
        merkle_root: B256::with_last_byte(1),
        zk_proof: vec![0],
    };

    assert_eq!(db.begin_ro()?.get_chain_info(chain_id)?, None);

    let mut tx = db.begin_rw()?;
    tx.insert_chain_info(chain_id, &chain_info)?;
    tx.commit()?;

    assert_eq!(db.begin_ro()?.get_chain_info(chain_id)?.unwrap(), chain_info);

    Ok(())
}
#[test]
fn node_get_insert_delete() -> Result<()> {
    let mut db = get_test_db();
    let node = Node::Null;
    let node_hash = node_hash(&node, alloy_rlp::encode(&node));

    assert_eq!(db.begin_ro()?.get_node(node_hash).unwrap_err(), ChainDbError::NodeNotFound);

    let mut tx = db.begin_rw()?;
    tx.insert_node(&node)?;
    tx.commit();

    assert_eq!(db.begin_ro()?.get_node(node_hash)?, node);

    let mut tx = db.begin_rw()?;
    tx.delete_node(node_hash)?;
    tx.commit();

    assert_eq!(db.begin_ro()?.get_node(node_hash).unwrap_err(), ChainDbError::NodeNotFound);

    Ok(())
}
