use core::future::Future;

use alloy_primitives::{B256, BlockNumber, U256};
use anyhow::{Context, anyhow};
use block_header::{EthBlockHeader, EvmBlockHeader};
use derive_new::new;
use ethers_core::types::{Block, BlockNumber as BlockTag};
use ethers_providers::{JsonRpcClient, Middleware};
use tokio::runtime::Handle;
use tracing::instrument;

use super::{BlockingProvider, EIP1186Proof, Result};

/// A provider that fetches data from an Ethereum node using the ethers crate.
#[derive(Debug, new)]
pub struct EthersProvider<T: JsonRpcClient> {
    client: ethers_providers::Provider<T>,
}

// Blocks current runtime to execute the future. Panics if called outside of the runtime
#[allow(clippy::expect_used)]
fn block_on<F: Future>(f: F) -> F::Output {
    let handle = Handle::try_current().expect("no tokio runtime");
    tokio::task::block_in_place(|| handle.block_on(f))
}

impl<T: JsonRpcClient> BlockingProvider for EthersProvider<T> {
    #[instrument(skip(self))]
    fn get_block_header(&self, block: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>> {
        let block = block_on(self.client.get_block(block))?;
        match block {
            Some(block) => {
                let eth_block_header = to_eth_block_header(block)?;
                Ok(Some(Box::new(eth_block_header) as Box<dyn EvmBlockHeader>))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    fn get_transaction_count(
        &self,
        address: alloy_primitives::Address,
        block: BlockNumber,
    ) -> Result<alloy_primitives::TxNumber> {
        let address = to_ethers_h160(address);
        let count = block_on(
            self.client
                .get_transaction_count(address, Some(block.into())),
        )?
        .as_u64();

        Ok(count)
    }

    #[instrument(skip(self))]
    fn get_balance(
        &self,
        address: alloy_primitives::Address,
        block: BlockNumber,
    ) -> Result<alloy_primitives::U256> {
        let address = to_ethers_h160(address);
        let balance = block_on(self.client.get_balance(address, Some(block.into())))?;
        Ok(from_ethers_u256(balance))
    }

    #[instrument(skip(self))]
    fn get_code(
        &self,
        address: alloy_primitives::Address,
        block: BlockNumber,
    ) -> Result<alloy_primitives::Bytes> {
        let address = to_ethers_h160(address);
        let code = block_on(self.client.get_code(address, Some(block.into())))?;
        Ok(from_ethers_bytes(code))
    }

    #[instrument(skip(self))]
    fn get_storage_at(
        &self,
        address: alloy_primitives::Address,
        key: alloy_primitives::StorageKey,
        block: BlockNumber,
    ) -> Result<alloy_primitives::StorageValue> {
        let address = to_ethers_h160(address);
        let key = to_ethers_h256(key);
        let value = block_on(self.client.get_storage_at(address, key, Some(block.into())))?;
        Ok(from_ethers_h256(value).into())
    }

    #[instrument(skip(self))]
    fn get_proof(
        &self,
        address: alloy_primitives::Address,
        storage_keys: Vec<alloy_primitives::StorageKey>,
        block: BlockNumber,
    ) -> Result<EIP1186Proof> {
        let address = to_ethers_h160(address);
        let storage_keys = storage_keys.into_iter().map(to_ethers_h256).collect();
        let proof = block_on(
            self.client
                .get_proof(address, storage_keys, Some(block.into())),
        )?;

        Ok(EIP1186Proof {
            address: address.0.into(),
            balance: from_ethers_u256(proof.balance),
            code_hash: from_ethers_h256(proof.code_hash),
            nonce: proof.nonce.as_u64(),
            storage_hash: from_ethers_h256(proof.storage_hash),
            account_proof: proof
                .account_proof
                .into_iter()
                .map(from_ethers_bytes)
                .collect(),
            storage_proof: proof.storage_proof.into_iter().map(Into::into).collect(),
        })
    }

    fn get_latest_block_number(&self) -> Result<BlockNumber> {
        Ok(block_on(self.client.get_block_number())?.as_u64())
    }
}

#[allow(clippy::expect_used)]
pub fn to_eth_block_header<T>(block: Block<T>) -> Result<EthBlockHeader> {
    let requests_hash = block.other.get("requestsHash").map(|v| {
        let requests_hash = v.as_str().expect("requestsHash should be a string");
        requests_hash
            .parse::<B256>()
            .expect("requestsHash should be a B256")
    });
    Ok(EthBlockHeader {
        parent_hash: from_ethers_h256(block.parent_hash),
        ommers_hash: from_ethers_h256(block.uncles_hash),
        beneficiary: block.author.context("author")?.0.into(),
        state_root: from_ethers_h256(block.state_root),
        transactions_root: from_ethers_h256(block.transactions_root),
        receipts_root: from_ethers_h256(block.receipts_root),
        logs_bloom: alloy_primitives::Bloom::from_slice(
            block.logs_bloom.context("logs bloom")?.as_bytes(),
        ),
        difficulty: from_ethers_u256(block.difficulty),
        number: block.number.context("number")?.as_u64(),
        gas_limit: block.gas_limit.as_u64(),
        gas_used: block.gas_used.as_u64(),
        timestamp: block.timestamp.as_u64(),
        extra_data: block.extra_data.0.into(),
        mix_hash: from_ethers_h256(block.mix_hash.context("mix_hash")?),
        nonce: block.nonce.context("nonce")?.0.into(),
        base_fee_per_gas: block
            .base_fee_per_gas
            .map(from_ethers_u256)
            .unwrap_or_default(),
        withdrawals_root: block.withdrawals_root.map(from_ethers_h256),
        blob_gas_used: block
            .blob_gas_used
            .map(TryInto::try_into)
            .transpose()
            .map_err(|e: &str| anyhow!(e))?,
        excess_blob_gas: block
            .excess_blob_gas
            .map(TryInto::try_into)
            .transpose()
            .map_err(|e: &str| anyhow!(e))?,
        parent_beacon_block_root: block.parent_beacon_block_root.map(from_ethers_h256),
        requests_hash,
    })
}

pub fn from_ethers_bytes(v: ethers_core::types::Bytes) -> alloy_primitives::Bytes {
    v.0.into()
}

pub fn to_ethers_h256(v: alloy_primitives::B256) -> ethers_core::types::H256 {
    v.0.into()
}

pub fn from_ethers_h256(v: ethers_core::types::H256) -> B256 {
    v.0.into()
}

pub const fn from_ethers_u256(v: ethers_core::types::U256) -> U256 {
    alloy_primitives::U256::from_limbs(v.0)
}

pub fn to_ethers_h160(v: alloy_primitives::Address) -> ethers_core::types::H160 {
    v.into_array().into()
}
