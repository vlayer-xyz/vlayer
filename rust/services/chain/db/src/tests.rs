use alloy_primitives::BlockNumber;
use anyhow::Result;
use block_trie::BlockTrie;
use common::GuestElf;
use key_value::InMemoryDatabase;
use mpt::{Sha2Node as Node, Sha2Trie as MerkleTrie, keccak256};
use rand::{RngCore, SeedableRng, rngs::StdRng};
use risc0_zkvm::{FakeReceipt, InnerReceipt, MaybePruned, Receipt, sha::Digest};

use super::*;

fn get_test_db() -> ChainDb {
    ChainDb::in_memory([GuestElf::default().id])
}

fn insert_node(db: &mut ChainDb, node_rlp: &Bytes) {
    let mut tx = db.begin_rw().expect("begin_rw failed");
    tx.insert_node(node_rlp).expect("insert_node failed");
    Box::new(tx).commit().expect("commit failed");
}

fn delete_node(db: &mut ChainDb, node_hash: B256) {
    let mut tx = db.begin_rw().expect("begin_rw failed");
    tx.delete_node(node_hash).expect("delete_node failed");
    Box::new(tx).commit().expect("commit failed");
}

// Fake block header to insert in MPT (must be big enough not to get inlined, so we can test if a tree is sparse)
fn block_header(block_num: u64) -> B256 {
    keccak256(alloy_rlp::encode(block_num))
}

fn insert_blocks(db: &mut ChainDb, blocks: impl IntoIterator<Item = BlockNumber>) -> (B256, Node) {
    let mut block_trie = BlockTrie::from_unchecked(MerkleTrie::new());
    for block_num in blocks {
        block_trie
            .insert_unchecked(block_num, &block_header(block_num))
            .expect("insert_unchecked failed");
    }

    let mut tx = db.begin_rw().expect("begin_rw failed");
    for node in &block_trie {
        tx.insert_node(&node).expect("insert_node failed");
    }
    Box::new(tx).commit().expect("commit failed");
    (block_trie.hash_slow(), block_trie.into_root())
}

fn check_proof(db: &ChainDb, root_hash: B256, block_num: u64) -> BlockTrie {
    let proof = db
        .get_merkle_proof(root_hash, block_num)
        .expect("get_merkle_proof failed");
    let merkle_trie: MerkleTrie = proof.into();
    let proof_trie = BlockTrie::from_unchecked(merkle_trie);
    assert_eq!(proof_trie.get(block_num).unwrap(), block_header(block_num));
    proof_trie
}

fn fake_proof() -> Bytes {
    let receipt = Receipt::new(
        InnerReceipt::Fake(FakeReceipt::new(MaybePruned::Pruned(Digest::ZERO))),
        vec![],
    );
    let chain_proof_receipt: ChainProofReceipt = receipt.into();
    (&chain_proof_receipt)
        .try_into()
        .expect("Failed to serialize receipt")
}

#[test]
fn read_only_error_on_write() -> Result<()> {
    let mut db =
        ChainDb::new(InMemoryDatabase::new(), Mode::ReadOnly, Box::new([GuestElf::default().id]));
    let res = db.begin_rw();
    // Not using .unwrap_err() because res is not Debug
    assert!(res.is_err_and(|e| e == ChainDbError::ReadOnly));
    Ok(())
}

#[test]
fn chain_info_get_insert() -> Result<()> {
    let mut db = get_test_db();
    let chain_id = 1;
    let chain_info = ChainInfo::new(
        NonEmptyRange::try_from_range(0..=2).unwrap(),
        B256::with_last_byte(1),
        fake_proof(),
    );

    assert_eq!(db.begin_ro()?.get_chain_info(chain_id)?, None);

    let mut tx = db.begin_rw()?;
    tx.upsert_chain_info(chain_id, &chain_info)?;
    Box::new(tx).commit()?;

    assert_eq!(db.begin_ro()?.get_chain_info(chain_id)?.unwrap(), chain_info);

    Ok(())
}

#[test]
fn node_get_insert_delete() -> Result<()> {
    let mut db = get_test_db();
    let node = Node::Null;
    let node_rlp = node.rlp_encoded();
    let node_hash = sha2(&node_rlp);

    assert_eq!(db.begin_ro()?.get_node(node_hash).unwrap_err(), ChainDbError::NodeNotFound);

    insert_node(&mut db, &node_rlp);
    let db_node = DbNode {
        hash: Some(node_hash),
        node,
        rlp: node_rlp,
    };
    assert_eq!(db.begin_ro()?.get_node(node_hash)?, db_node);

    delete_node(&mut db, node_hash);
    assert_eq!(db.begin_ro()?.get_node(node_hash).unwrap_err(), ChainDbError::NodeNotFound);

    Ok(())
}

#[test]
fn insert_same_node_twice() -> Result<()> {
    let mut db = get_test_db();
    let node_rlp = Node::Null.rlp_encoded();
    // Inserting the same node twice should not raise any error
    insert_node(&mut db, &node_rlp);
    insert_node(&mut db, &node_rlp);
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
    insert_node(&mut db, &Node::Null.rlp_encoded());
    assert_eq!(
        db.get_merkle_proof(Node::Null.hash_slow(), 0).unwrap_err(),
        ChainDbError::BlockNotFound
    );
    Ok(())
}

#[test]
fn proof_one_node() -> Result<()> {
    let mut db = get_test_db();

    let (root_hash, root) = insert_blocks(&mut db, vec![0]);
    let proof = db.get_merkle_proof(root_hash, 0)?;
    let proof_trie: MerkleTrie = proof.into();
    assert_eq!(proof_trie.0, root);

    Ok(())
}

#[test]
#[should_panic(expected = "Attempted to access unresolved node")]
fn proof_extension() {
    let mut db = get_test_db();

    let (root_hash, _) = insert_blocks(&mut db, vec![0, 1_000_000]);
    let proof_trie = check_proof(&db, root_hash, 1_000_000);

    // The tree should be sparse - block 0 not included
    proof_trie.get(0);
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

#[test]
fn get_chain_trie_inner() -> Result<()> {
    let mut db = get_test_db();

    let (root_hash, _) = insert_blocks(&mut db, 0..=10);
    let chain_info =
        ChainInfo::new(NonEmptyRange::try_from_range(0..=10).unwrap(), root_hash, fake_proof());

    let mut tx = db.begin_rw()?;
    tx.upsert_chain_info(1, &chain_info)?;
    Box::new(tx).commit()?;

    let chain_trie = db.get_chain_trie_inner(1)?.unwrap();
    assert_eq!(chain_trie.block_range, (0..=10));
    assert_eq!(chain_trie.trie.hash_slow(), root_hash);

    Ok(())
}

#[test]
fn update_chain() -> Result<()> {
    let mut db = get_test_db();

    let mut trie = BlockTrie::from_unchecked(MerkleTrie::new());

    trie.insert_unchecked(1, &block_header(1))?;
    trie.insert_unchecked(2, &block_header(2))?;

    let root_hash = trie.hash_slow();
    let rlp_nodes = (&trie).into_iter();
    let chain_info =
        ChainInfo::new(NonEmptyRange::try_from_range(1..=3).unwrap(), root_hash, fake_proof());

    db.update_chain(0, ChainUpdate::new(chain_info, &trie, []))?;
    for block_num in [1, 2] {
        check_proof(&db, root_hash, block_num);
    }
    trie.insert_unchecked(0, &block_header(0))?;
    trie.insert_unchecked(3, &block_header(3))?;
    let new_root_hash = trie.hash_slow();
    let (added_nodes, removed_nodes) = difference(rlp_nodes, &trie);
    let chain_info =
        ChainInfo::new(NonEmptyRange::try_from_range(0..=2).unwrap(), new_root_hash, fake_proof());

    db.update_chain(0, ChainUpdate::new(chain_info, added_nodes, removed_nodes.clone()))?;
    for block_num in [0, 1, 2, 3] {
        check_proof(&db, new_root_hash, block_num);
    }

    assert!(!removed_nodes.is_empty());
    for node in removed_nodes {
        assert_eq!(db.begin_ro()?.get_node(sha2(node)).unwrap_err(), ChainDbError::NodeNotFound);
    }

    Ok(())
}
