use std::fmt::Debug;

use alloy_primitives::{Address, B256, Bytes, StorageKey, StorageValue, TxNumber, U256};
use ethers_core::types::StorageProof as EthersStorageProof;
use serde::{Deserialize, Serialize};

use crate::ethers::{from_ethers_bytes, from_ethers_u256};

/// Data structure with proof for one single storage-entry
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StorageProof {
    pub key: StorageKey,
    pub proof: Vec<Bytes>,
    pub value: StorageValue,
}

impl From<EthersStorageProof> for StorageProof {
    fn from(proof: EthersStorageProof) -> Self {
        StorageProof {
            key: from_ethers_u256(proof.key).to_be_bytes().into(),
            proof: proof.proof.into_iter().map(from_ethers_bytes).collect(),
            value: from_ethers_u256(proof.value),
        }
    }
}

/// Response for EIP-1186 account proof `eth_getProof`
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EIP1186Proof {
    pub address: Address,
    pub balance: U256,
    pub code_hash: B256,
    pub nonce: TxNumber,
    pub storage_hash: B256,
    pub account_proof: Vec<Bytes>,
    pub storage_proof: Vec<StorageProof>,
}
