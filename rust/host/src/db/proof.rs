use super::provider::ProviderDb;
use crate::{proof::EIP1186Proof, provider::Provider};
use alloy_primitives::{Address, Bytes, B256, U256};
use anyhow::Context;
use mpt::MerkleTrie;
use revm::{
    primitives::{AccountInfo, Bytecode, HashMap, HashSet},
    Database,
};
use std::rc::Rc;

/// A revm [Database] backed by a [Provider] that caches all queries needed for a state proof.
pub struct ProofDb<P> {
    accounts: HashMap<Address, HashSet<U256>>,
    contracts: HashMap<B256, Bytes>,
    // Numbers of all block hashes requested by `blockhash(number)` calls.
    block_hash_numbers: HashSet<U256>,

    db: ProviderDb<P>,
}

impl<P: Provider> Database for ProofDb<P> {
    type Error = <ProviderDb<P> as Database>::Error;

    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let basic = self.db.basic(address)?;
        self.accounts.entry(address).or_default();

        Ok(basic)
    }

    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        let code = self.db.code_by_hash(code_hash)?;
        self.contracts.insert(code_hash, code.original_bytes());

        Ok(code)
    }

    fn storage(&mut self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let storage = self.db.storage(address, index)?;
        self.accounts.entry(address).or_default().insert(index);

        Ok(storage)
    }

    fn block_hash(&mut self, number: U256) -> Result<B256, Self::Error> {
        let block_hash = self.db.block_hash(number)?;
        self.block_hash_numbers.insert(number);

        Ok(block_hash)
    }
}

impl<P: Provider> ProofDb<P> {
    pub fn new(provider: Rc<P>, block_number: u64) -> Self {
        Self {
            accounts: HashMap::new(),
            contracts: HashMap::new(),
            block_hash_numbers: HashSet::new(),
            db: ProviderDb::new(provider, block_number),
        }
    }

    pub fn contracts(&self) -> Vec<Bytes> {
        self.contracts.values().cloned().collect()
    }

    pub fn fetch_ancestors(&self) -> anyhow::Result<Vec<P::Header>> {
        let provider = &self.db.provider;
        let mut ancestors = Vec::new();
        if let Some(block_hash_min_number) = self.block_hash_numbers.iter().min() {
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
        let mut proofs = Vec::new();
        for (address, storage_keys) in &self.accounts {
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
