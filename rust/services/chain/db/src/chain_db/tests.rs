use crate::in_memory::InMemoryDatabase;
use alloy_primitives::BlockNumber;
use anyhow::Result;
use mpt::MerkleTrie;

use super::*;

fn get_test_db() -> ChainDb<InMemoryDatabase> {
    let db = InMemoryDatabase::new();
    ChainDb::new(db)
}

fn insert_node(db: &mut ChainDb<InMemoryDatabase>, node_rlp: Bytes) {
    let mut tx = db.begin_rw().expect("begin_rw failed");
    tx.insert_node(node_rlp).expect("insert_node failed");
    tx.commit().expect("commit failed");
}

fn delete_node(db: &mut ChainDb<InMemoryDatabase>, node_hash: B256) {
    let mut tx = db.begin_rw().expect("begin_rw failed");
    tx.delete_node(node_hash).expect("delete_node faile");
    tx.commit().expect("commit failed");
}

// Fake block header to insert in MPT (must be big enough not get inlined, so we can test if a tree is sparse)
fn block_header(block_num: u64) -> Bytes {
    keccak256(alloy_rlp::encode(block_num)).into()
}

fn insert_blocks(
    db: &mut ChainDb<InMemoryDatabase>,
    blocks: impl IntoIterator<Item = BlockNumber>,
) -> (B256, Node) {
    let mut mpt = MerkleTrie::new();
    for block_num in blocks {
        mpt.insert(alloy_rlp::encode(block_num), block_header(block_num))
            .expect("insert failed");
    }

    let mut tx = db.begin_rw().expect("begin_rw failed");
    for node_rlp in mpt.to_rlp_nodes() {
        tx.insert_node(node_rlp).expect("insert_node failed");
    }
    tx.commit().expect("commit failed");
    (mpt.hash_slow(), mpt.0)
}

#[test]
fn chain_info_get_insert() -> Result<()> {
    let mut db = get_test_db();
    let chain_id = 1;
    let chain_info = ChainInfo {
        first_block: 0,
        last_block: 1,
        merkle_root: B256::with_last_byte(1),
        zk_proof: Bytes::from_static(&[0]),
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
    let node_rlp = node.rlp_encoded();
    let node_hash = keccak256(&node_rlp);

    assert_eq!(db.begin_ro()?.get_node(node_hash).unwrap_err(), ChainDbError::NodeNotFound);

    insert_node(&mut db, node_rlp);
    assert_eq!(db.begin_ro()?.get_node(node_hash)?, node);

    delete_node(&mut db, node_hash);
    assert_eq!(db.begin_ro()?.get_node(node_hash).unwrap_err(), ChainDbError::NodeNotFound);

    Ok(())
}

#[test]
fn proof_empty_db() -> Result<()> {
    let db = get_test_db();
    assert_eq!(
        db.get_merkle_proof(B256::with_last_byte(1), 0).unwrap_err(),
        ChainDbError::NodeNotFound
    );
    Ok(())
}

#[test]
fn proof_empty_root() -> Result<()> {
    let mut db = get_test_db();
    insert_node(&mut db, Node::Null.rlp_encoded());
    assert_eq!(
        db.get_merkle_proof(EMPTY_ROOT_HASH, 0).unwrap_err(),
        ChainDbError::BlockNotFound
    );
    Ok(())
}

#[test]
fn proof_one_node() -> Result<()> {
    let mut db = get_test_db();

    let (root_hash, root) = insert_blocks(&mut db, vec![0]);
    let proof = db.get_merkle_proof(root_hash, 0)?;
    let proof_trie: MerkleTrie = proof.into_vec().into_iter().collect();
    assert_eq!(proof_trie.0, root);

    Ok(())
}

#[test]
fn proof_extension() -> Result<()> {
    let mut db = get_test_db();

    let (root_hash, root) = insert_blocks(&mut db, vec![0, 1_000_000]);
    let proof = db.get_merkle_proof(root_hash, 1_000_000)?;
    let proof_trie: MerkleTrie = proof.into_vec().into_iter().collect();
    assert_eq!(
        proof_trie.get(alloy_rlp::encode(1_000_000u64)).unwrap(),
        &block_header(1_000_000u64)
    );

    // The tree should be sparse - block 0 not included
    let res = std::panic::catch_unwind(|| proof_trie.get(alloy_rlp::encode(0u64)));
    assert!(res.is_err());

    Ok(())
}
