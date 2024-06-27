use super::provider::ProviderDb;
use crate::provider::Provider;
use alloy_primitives::{Address, Bytes, B256, U256};
use revm::{
    primitives::{AccountInfo, Bytecode, HashMap, HashSet},
    Database,
};

/// A revm [Database] backed by a [Provider] that caches all queries needed for a state proof.
pub struct ProofDb<P> {
    accounts: HashMap<Address, HashSet<U256>>,
    contracts: HashMap<B256, Bytes>,
    block_hash_numbers: HashSet<U256>,

    db: ProviderDb<P>,
}

impl<P: Provider> ProofDb<P> {
    pub fn new(provider: P, block_number: u64) -> Self {
        Self {
            accounts: HashMap::new(),
            contracts: HashMap::new(),
            block_hash_numbers: HashSet::new(),
            db: ProviderDb::new(provider, block_number),
        }
    }

    pub fn provider(&self) -> &P {
        &self.db.provider
    }
    pub fn block_number(&self) -> u64 {
        self.db.block_number
    }
    pub fn accounts(&self) -> &HashMap<Address, HashSet<U256>> {
        &self.accounts
    }
    pub fn contracts(&self) -> &HashMap<B256, Bytes> {
        &self.contracts
    }
    pub fn block_hash_numbers(&self) -> &HashSet<U256> {
        &self.block_hash_numbers
    }
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
