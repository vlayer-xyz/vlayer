use super::provider::{ProviderDb, ProviderDbError};
use crate::{proof::EIP1186Proof, provider::BlockingProvider};
use alloy_primitives::{Address, Bytes, B256, U256};
use anyhow::Context;
use mpt::MerkleTrie;
use revm::{
    primitives::{AccountInfo, Bytecode, HashMap, HashSet},
    DatabaseRef,
};
use std::{cell::RefCell, rc::Rc};
use vlayer_engine::block_header::EvmBlockHeader;

#[derive(Default, Debug)]
struct State {
    accounts: HashMap<Address, HashSet<U256>>,
    contracts: HashMap<B256, Bytes>,
    // Numbers of all block hashes requested by `blockhash(number)` calls.
    block_hash_numbers: HashSet<U256>,
}

/// A revm [Database] backed by a [Provider] that caches all queries needed for a state proof.
pub struct ProofDb<P>
where
    P: BlockingProvider,
{
    db: ProviderDb<P>,
    state: RefCell<State>,
}

impl<P> DatabaseRef for ProofDb<P>
where
    P: BlockingProvider,
{
    type Error = ProviderDbError<P::Error>;

    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let basic = self.db.basic_ref(address)?;
        let mut state = self.state.borrow_mut();
        state.accounts.entry(address).or_default();

        Ok(basic)
    }

    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        let code = self.db.code_by_hash_ref(code_hash)?;
        let mut state = self.state.borrow_mut();
        state.contracts.insert(code_hash, code.original_bytes());

        Ok(code)
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let storage = self.db.storage_ref(address, index)?;
        let mut state = self.state.borrow_mut();
        state.accounts.entry(address).or_default().insert(index);

        Ok(storage)
    }

    fn block_hash_ref(&self, number: U256) -> Result<B256, Self::Error> {
        let block_hash = self.db.block_hash_ref(number)?;
        let mut state = self.state.borrow_mut();
        state.block_hash_numbers.insert(number);

        Ok(block_hash)
    }
}

impl<P> ProofDb<P>
where
    P: BlockingProvider,
{
    pub fn new(provider: Rc<P>, block_number: u64) -> Self {
        let state = RefCell::new(State::default());
        Self {
            state,
            db: ProviderDb::new(provider, block_number),
        }
    }

    pub fn contracts(&self) -> Vec<Bytes> {
        let state = self.state.borrow();
        state.contracts.values().cloned().collect()
    }

    pub fn fetch_ancestors(&self) -> anyhow::Result<Vec<Box<dyn EvmBlockHeader>>> {
        let state = self.state.borrow();
        let provider = &self.db.provider;
        let mut ancestors = Vec::new();
        if let Some(block_hash_min_number) = state.block_hash_numbers.iter().min() {
            let block_hash_min_number: u64 = block_hash_min_number.to();
            for number in (block_hash_min_number..self.db.block_number).rev() {
                let header = provider
                    .get_block_header(number)?
                    .with_context(|| format!("block {number} not found"))?;
                ancestors.push(header);
            }
        }
        Ok(ancestors)
    }

    pub fn prepare_state_storage_tries(&self) -> anyhow::Result<(MerkleTrie, Vec<MerkleTrie>)> {
        let proofs = self.fetch_proofs()?;
        let state_trie = Self::state_trie(&proofs)?;
        let storage_tries = Self::storage_tries(&proofs)?;
        Ok((state_trie, storage_tries))
    }

    fn fetch_proofs(&self) -> anyhow::Result<Vec<EIP1186Proof>> {
        let state = self.state.borrow();
        let mut proofs = Vec::new();
        for (address, storage_keys) in &state.accounts {
            let proof = self.db.provider.get_proof(
                *address,
                storage_keys.iter().map(|v| B256::from(*v)).collect(),
                self.db.block_number,
            )?;
            proofs.push(proof);
        }
        Ok(proofs)
    }

    fn state_trie(proofs: &[EIP1186Proof]) -> anyhow::Result<MerkleTrie> {
        let state_nodes = proofs.iter().flat_map(|p| p.account_proof.iter());
        let state_trie =
            MerkleTrie::from_rlp_nodes(state_nodes).context("invalid account proof")?;
        Ok(state_trie)
    }

    fn storage_tries(proofs: &[EIP1186Proof]) -> anyhow::Result<Vec<MerkleTrie>> {
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
        Ok(storage_tries)
    }
}
