use alloy_sol_types::private::{Address, Bytes, U256};
use ethers_core::types::BlockNumber as BlockTag;
use ethers_core::utils::keccak256;
use forge::revm::primitives::alloy_primitives::{
    BlockNumber, ChainId, StorageKey, StorageValue, TxNumber,
};
use forge::revm::primitives::{b256, Account, Bytecode};
use forge::revm::{Database, Evm};
use forge::revm::{DatabaseRef, JournaledState};
use foundry_evm_core::backend::{Backend, FoundryEvmInMemoryDB};
use host::host::error::HostError;
use host::proof::EIP1186Proof;
use host::provider::factory::ProviderFactory;
use host::provider::BlockingProvider;
use std::convert::Infallible;
use vlayer_engine::block_header::eth::EthBlockHeader;
use vlayer_engine::block_header::EvmBlockHeader;

pub struct MockProvider {
    state: JournaledState,
}

impl MockProvider {
    fn try_get_account(&self, address: Address) -> Option<Account> {
        self.state
            .state
            .get(&address)
            .map(|account| account.clone())
    }
}

impl<'a> BlockingProvider for MockProvider {
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
            state_root: b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
            ..EthBlockHeader::default()
        })))
    }

    fn get_code(&self, address: Address, _block: BlockNumber) -> Result<Bytes, Self::Error> {
        let account = self.state.account(address);
        Ok(account
            .info
            .code
            .clone()
            .map_or(Bytes::default(), |code| code.bytes()))
    }

    fn get_proof(
        &self,
        address: Address,
        _storage_keys: Vec<StorageKey>,
        _block: BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error> {
        let Some(account) = self.try_get_account(address) else {
            return Ok(EIP1186Proof::default());
        };

        Ok(EIP1186Proof {
            balance: account.info.balance,
            address,
            nonce: account.info.nonce,
            code_hash: account.info.code_hash,
            ..EIP1186Proof::default()
        })
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
        _address: Address,
        _block: BlockNumber,
    ) -> Result<TxNumber, Self::Error> {
        dbg!("get_transaction_count");
        Ok(0)
    }
}

pub struct MockProviderFactory {
    pub state: JournaledState,
}

impl ProviderFactory<MockProvider> for MockProviderFactory {
    fn create(&self, _chain_id: ChainId) -> Result<MockProvider, HostError> {
        Ok(MockProvider {
            state: self.state.clone(),
        })
    }
}
