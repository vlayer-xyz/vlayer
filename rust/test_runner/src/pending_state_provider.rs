use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::Infallible;

use alloy_sol_types::private::{Address, Bytes, U256};
use alloy_trie::proof::ProofRetainer;
use alloy_trie::{HashBuilder, Nibbles};
use ethers_core::types::BlockNumber as BlockTag;
use ethers_core::utils::keccak256;
use forge::revm::primitives::alloy_primitives::private::alloy_rlp;
use forge::revm::primitives::alloy_primitives::private::alloy_rlp::Encodable;
use forge::revm::primitives::alloy_primitives::{
    BlockNumber, ChainId, StorageKey, StorageValue, TxNumber,
};
use forge::revm::primitives::{Account, AccountInfo, EvmStorageSlot, FixedBytes, B256};
use forge::revm::Database;
use forge::revm::{DatabaseRef, JournaledState};

use host::db::proof::ProofDb;
use host::host::error::HostError;
use host::proof::{EIP1186Proof, StorageProof};
use host::provider::factory::ProviderFactory;
use host::provider::BlockingProvider;
use vlayer_engine::block_header::eth::EthBlockHeader;
use vlayer_engine::block_header::EvmBlockHeader;

pub struct PendingStateProvider {
    state: JournaledState,
}

impl PendingStateProvider {
    fn try_get_account(&self, address: Address) -> Option<Account> {
        self.state
            .state
            .get(&address)
            .map(|account| account.clone())
    }

    fn proofs(&self) -> anyhow::Result<Vec<EIP1186Proof>> {
        let state = self.state.state.borrow();
        let mut proofs = Vec::new();
        for (address, account) in state {
            let proof = self.get_proof(
                *address,
                account.storage.keys().map(|v| B256::from(*v)).collect(),
                0,
            )?;
            proofs.push(proof);
        }
        Ok(proofs)
    }

    fn get_state_root(&self) -> anyhow::Result<B256> {
        let proofs = self.proofs()?;
        let state_trie = ProofDb::<PendingStateProvider>::state_trie(&proofs)?;
        Ok(state_trie.hash_slow())
    }
}

impl<'a> BlockingProvider for PendingStateProvider {
    type Error = Infallible;

    fn get_balance(&self, address: Address, _block: BlockNumber) -> Result<U256, Self::Error> {
        Ok(self.state.account(address).info.balance)
    }

    fn get_block_header(
        &self,
        _block: BlockTag,
    ) -> Result<Option<Box<dyn EvmBlockHeader>>, Self::Error> {
        Ok(Some(Box::new(EthBlockHeader {
            number: 15537395,
            state_root: self.get_state_root().unwrap_or_default(),
            ..EthBlockHeader::default()
        })))
    }

    fn get_code(&self, address: Address, _block: BlockNumber) -> Result<Bytes, Self::Error> {
        let account = self.state.account(address);
        Ok(account
            .info
            .code
            .clone()
            .map_or(Bytes::default(), |code| code.original_bytes()))
    }

    fn get_proof(
        &self,
        address: Address,
        _storage_keys: Vec<StorageKey>,
        _block: BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error> {
        dbg!(self.try_get_account(address));
        let Some(_) = self.try_get_account(address) else {
            return Ok(EIP1186Proof::default());
        };

        Ok(prove_account_at(&self.state, address, _storage_keys))
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        _block: BlockNumber,
    ) -> Result<StorageValue, Self::Error> {
        self.try_get_account(address)
            .map_or(Ok(StorageValue::default()), |account| {
                Ok(account
                    .storage
                    .get(&key.into())
                    .map_or(StorageValue::default(), |value| value.present_value.into()))
            })
    }

    fn get_transaction_count(
        &self,
        address: Address,
        _block: BlockNumber,
    ) -> Result<TxNumber, Self::Error> {
        Ok(self.try_get_account(address).unwrap_or_default().info.nonce)
    }
}

pub struct PendingStateProviderFactory {
    pub state: JournaledState,
}

impl ProviderFactory<PendingStateProvider> for PendingStateProviderFactory {
    fn create(&self, _chain_id: ChainId) -> Result<PendingStateProvider, HostError> {
        Ok(PendingStateProvider {
            state: self.state.clone(),
        })
    }
}

fn prove_account_at(
    journaled_state: &JournaledState,
    address: Address,
    keys: Vec<B256>,
) -> EIP1186Proof {
    let state = journaled_state.state.clone();
    let account = journaled_state.account(address);

    let mut builder =
        HashBuilder::default().with_proof_retainer(ProofRetainer::new(vec![Nibbles::unpack(
            keccak256(address),
        )]));

    for (key, account) in trie_accounts(&state) {
        builder.add_leaf(key, &account);
    }

    let _ = builder.root();

    let proof = builder.take_proofs().values().cloned().collect::<Vec<_>>();
    let storage_proofs = prove_storage(&account.storage, &keys);

    let account_proof = EIP1186Proof {
        address,
        balance: account.info.balance,
        nonce: account.info.nonce,
        code_hash: account.info.code_hash,
        storage_hash: storage_root(&account.storage),
        account_proof: proof.into(),
        storage_proof: keys
            .into_iter()
            .zip(storage_proofs)
            .map(|(key, proof)| {
                let storage_key: U256 = key.into();
                let value = account
                    .storage
                    .get(&storage_key)
                    .cloned()
                    .unwrap_or_default()
                    .present_value;
                StorageProof { key, value, proof }
            })
            .collect(),
    };

    account_proof
}

pub fn trie_accounts(accounts: &HashMap<Address, Account>) -> Vec<(Nibbles, Vec<u8>)> {
    let mut accounts = accounts
        .iter()
        .map(|(address, account)| {
            let data = trie_account_rlp(&account.info, &account.storage);
            (
                Nibbles::unpack(alloy_sol_types::private::keccak256(*address)),
                data,
            )
        })
        .collect::<Vec<_>>();
    accounts.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));

    accounts
}

pub fn trie_account_rlp(info: &AccountInfo, storage: &HashMap<U256, EvmStorageSlot>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let list: [&dyn Encodable; 4] = [
        &info.nonce,
        &info.balance,
        &storage_root(storage),
        &info.code_hash,
    ];

    alloy_rlp::encode_list::<_, dyn Encodable>(&list, &mut out);

    out
}

pub fn storage_root(storage: &HashMap<U256, EvmStorageSlot>) -> B256 {
    build_root(trie_storage(storage))
}

pub fn build_root(values: impl IntoIterator<Item = (Nibbles, Vec<u8>)>) -> B256 {
    let mut builder = HashBuilder::default();
    for (key, value) in values {
        builder.add_leaf(key, value.as_ref());
    }
    builder.root()
}

pub fn trie_storage(storage: &HashMap<U256, EvmStorageSlot>) -> Vec<(Nibbles, Vec<u8>)> {
    let mut storage = storage
        .iter()
        .map(|(key, value)| {
            let data = alloy_rlp::encode(value.present_value);
            (
                Nibbles::unpack(alloy_sol_types::private::keccak256(key.to_be_bytes::<32>())),
                data,
            )
        })
        .collect::<Vec<_>>();
    storage.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));

    storage
}

pub fn prove_storage(
    storage: &HashMap<U256, EvmStorageSlot>,
    keys: &Vec<FixedBytes<32>>,
) -> Vec<Vec<Bytes>> {
    let keys: Vec<_> = keys
        .iter()
        .map(|key| Nibbles::unpack(alloy_sol_types::private::keccak256(key)))
        .collect();

    let mut builder = HashBuilder::default().with_proof_retainer(ProofRetainer::new(keys.clone()));

    for (key, value) in trie_storage(storage) {
        builder.add_leaf(key, &value);
    }

    let _ = builder.root();

    let mut proofs = Vec::new();
    let all_proof_nodes = builder.take_proofs();

    for proof_key in keys {
        // Iterate over all proof nodes and find the matching ones.
        // The filtered results are guaranteed to be in order.
        let matching_proof_nodes = all_proof_nodes
            .iter()
            .filter(|(path, _)| proof_key.starts_with(path))
            .map(|(_, node)| node.clone());
        proofs.push(matching_proof_nodes.collect());
    }

    proofs
}
