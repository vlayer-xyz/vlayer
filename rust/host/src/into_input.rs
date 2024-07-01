use crate::db::proof::ProofDb;
use crate::provider::Provider;
use alloy_primitives::{Sealed, B256};
use anyhow::{ensure, Context};
use log::debug;
use revm::primitives::HashMap;
use vlayer_engine::evm::block_header::EvmBlockHeader;
use vlayer_engine::{evm::input::EvmInput, MerkleTrie};

pub fn into_input<P: Provider>(
    db: ProofDb<P>,
    header: Sealed<P::Header>,
) -> anyhow::Result<EvmInput<P::Header>> {
    let db = &db;

    // use the same provider as the database
    let provider = db.provider();

    // retrieve EIP-1186 proofs for all accounts
    let mut proofs = Vec::new();
    for (address, storage_keys) in db.accounts() {
        let proof = provider.get_proof(
            *address,
            storage_keys.iter().map(|v| B256::from(*v)).collect(),
            db.block_number(),
        )?;
        proofs.push(proof);
    }

    // build the sparse MPT for the state and verify against the header
    let state_nodes = proofs.iter().flat_map(|p| p.account_proof.iter());
    let state_trie = MerkleTrie::from_rlp_nodes(state_nodes).context("invalid account proof")?;
    ensure!(
        header.state_root() == &state_trie.hash_slow(),
        "root of the state trie does not match the header"
    );

    // build the sparse MPT for account storages and filter duplicates
    let mut storage_tries = HashMap::new();
    for proof in proofs {
        // skip non-existing accounts or accounts where no storage slots were requested
        if proof.storage_proof.is_empty() || proof.storage_hash.is_zero() {
            continue;
        }

        let storage_nodes = proof.storage_proof.iter().flat_map(|p| p.proof.iter());
        let storage_trie =
            MerkleTrie::from_rlp_nodes(storage_nodes).context("invalid storage proof")?;
        storage_tries.insert(storage_trie.hash_slow(), storage_trie);
    }
    let storage_tries: Vec<_> = storage_tries.into_values().collect();

    // collect the bytecode of all referenced contracts
    let contracts: Vec<_> = db.contracts().values().cloned().collect();

    // retrieve ancestor block headers
    let mut ancestors = Vec::new();
    if let Some(block_hash_min_number) = db.block_hash_numbers().iter().min() {
        let block_hash_min_number: u64 = block_hash_min_number.to();
        for number in (block_hash_min_number..db.block_number()).rev() {
            let header = provider
                .get_block_header(number)?
                .with_context(|| format!("block {number} not found"))?;
            ancestors.push(header);
        }
    }

    let header = header.into_inner();
    let evm_input = EvmInput {
        header,
        state_trie,
        storage_tries,
        contracts,
        ancestors,
    };
    evm_input.print_sizes();

    Ok(evm_input)
}
