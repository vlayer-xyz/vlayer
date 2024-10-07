use std::collections::HashSet;

use alloy_primitives::BlockNumber;
use anyhow::Result;
use chain_engine::BlockTrie;
use mpt::MerkleTrie;
use rand::{rngs::StdRng, RngCore, SeedableRng};

use super::*;
use crate::in_memory::InMemoryDatabase;

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
    tx.delete_node(node_hash).expect("delete_node failed");
    tx.commit().expect("commit failed");
}

// Fake block hash that is just a hash of the RLP-encoded block number
fn block_hash(block_num: u64) -> B256 {
    keccak256(alloy_rlp::encode(block_num))
}

fn insert_blocks(
    db: &mut ChainDb<InMemoryDatabase>,
    blocks: impl IntoIterator<Item = BlockNumber>,
) -> (B256, Node) {
    let mut trie = BlockTrie::new();
    for block_num in blocks {
        trie.insert(block_num, &block_hash(block_num));
    }

    let mut tx = db.begin_rw().expect("begin_rw failed");
    for node_rlp in &trie {
        tx.insert_node(node_rlp).expect("insert_node failed");
    }
    tx.commit().expect("commit failed");
    (trie.hash_slow(), trie.into_root())
}

fn check_proof(db: &ChainDb<InMemoryDatabase>, root_hash: B256, block_num: u64) -> BlockTrie {
    let proof = db
        .get_merkle_proof(root_hash, block_num)
        .expect("get_merkle_proof failed");
    let proof_trie: BlockTrie = proof.into_vec().into_iter().collect::<MerkleTrie>().into();
    assert_eq!(proof_trie.get(block_num).unwrap(), &block_hash(block_num));
    proof_trie
}

static EMPTY_PROOF: &[u8] = &[];

#[test]
fn chain_info_get_insert() -> Result<()> {
    let mut db = get_test_db();
    let chain_id = 1;
    let chain_info = ChainInfo::new((0..2), B256::with_last_byte(1), EMPTY_PROOF);

    assert_eq!(db.begin_ro()?.get_chain_info(chain_id)?, None);

    let mut tx = db.begin_rw()?;
    tx.upsert_chain_info(chain_id, &chain_info)?;
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
    let proof_trie = check_proof(&db, root_hash, 1_000_000);

    // The tree should be sparse - block 0 not included
    let res = std::panic::catch_unwind(|| proof_trie.get(0));
    assert!(res.is_err());

    Ok(())
}

#[test]
fn proof_random_blocks() -> Result<()> {
    let mut db = get_test_db();

    let mut rng = StdRng::seed_from_u64(0);
    let blocks: Vec<u64> = (0..100).map(|_| rng.next_u64()).collect();
    let (root_hash, _) = insert_blocks(&mut db, blocks.iter().cloned());

    for block_num in blocks {
        check_proof(&db, root_hash, block_num);
    }

    Ok(())
}

fn trie_and_chain_info(blocks: &[BlockNumber]) -> (BlockTrie, B256, ChainInfo) {
    let block_trie: BlockTrie = blocks
        .iter()
        .map(|block_num| (*block_num, block_hash(*block_num)))
        .collect();
    let root_hash = block_trie.hash_slow();
    let chain_info =
        ChainInfo::new((blocks[0]..*blocks.last().unwrap() + 1), root_hash, EMPTY_PROOF);
    (block_trie, root_hash, chain_info)
}

fn update_chain(
    db: &mut ChainDb<InMemoryDatabase>,
    chain_id: ChainId,
    chain_info: &ChainInfo,
    added_nodes: impl IntoIterator<Item = Bytes>,
    removed_nodes: impl IntoIterator<Item = Bytes>,
) {
    let chain_update = ChainUpdate::new(chain_info.clone(), added_nodes, removed_nodes);
    db.update_chain(chain_id, chain_update)
        .expect("update_chain_failed");
}

#[test]
fn test_update_chain() -> Result<()> {
    let mut db = get_test_db();

    let (block_trie, root_hash, chain_info) = trie_and_chain_info(&[0]);
    update_chain(&mut db, 0, &chain_info, &block_trie, []);
    assert_eq!(db.get_chain_info(0)?.unwrap(), chain_info);
    assert_eq!(db.begin_ro()?.get_node(root_hash)?, block_trie.clone().into_root());

    let (new_block_trie, new_root_hash, chain_info) = trie_and_chain_info(&[1]);
    update_chain(&mut db, 0, &chain_info, &new_block_trie, &block_trie);
    assert_eq!(db.get_chain_info(0)?.unwrap(), chain_info);
    assert_eq!(db.begin_ro()?.get_node(new_root_hash)?, new_block_trie.into_root());
    assert_eq!(db.begin_ro()?.get_node(root_hash).unwrap_err(), ChainDbError::NodeNotFound);

    Ok(())
}
