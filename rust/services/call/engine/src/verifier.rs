use alloy_primitives::{BlockNumber, B256};
use auto_impl::auto_impl;
use block_header::Hashable;
use chain_common::ChainProof;
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::{guest::env, sha::Digest, Receipt};
use static_assertions::assert_obj_safe;

use crate::evm::input::MultiEvmInput;

#[auto_impl(Fn)]
pub trait ZkProofVerifier: Send + Sync + 'static {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result<(), VerificationError>;
}

assert_obj_safe!(ZkProofVerifier);

pub struct GuestVerifier;

impl ZkProofVerifier for GuestVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result<(), VerificationError> {
        env::verify(elf_id, receipt.journal.as_ref()).expect("infallible");
        Ok(())
    }
}

pub struct HostVerifier;

impl ZkProofVerifier for HostVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result<(), VerificationError> {
        receipt.verify(elf_id)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChainProofError {
    #[error("Receipt deserialization error: {0}")]
    Receipt(#[from] bincode::Error),
    #[error("ZK verification error: {0}")]
    Zk(#[from] VerificationError),
    #[error("Journal decoding error: {0}")]
    Journal(#[from] risc0_zkvm::serde::Error),
    #[error("ELF ID mismatch: expected={expected} got={got}")]
    ElfId { expected: Digest, got: Digest },
    #[error("Root hash mismatch: proven={proven} actual={actual}")]
    RootHash { proven: B256, actual: B256 },
}

#[auto_impl(Fn)]
pub trait ChainProofVerifier: Send + Sync + 'static {
    fn verify(&self, proof: &ChainProof) -> Result<(), ChainProofError>;
}

assert_obj_safe!(ChainProofVerifier);

pub struct ZkChainProofVerifier {
    chain_guest_id: Digest,
    zk_verifier: Box<dyn ZkProofVerifier>,
}

impl ZkChainProofVerifier {
    #[must_use]
    pub fn new(chain_guest_id: impl Into<Digest>, zk_verifier: impl ZkProofVerifier) -> Self {
        Self {
            chain_guest_id: chain_guest_id.into(),
            zk_verifier: Box::new(zk_verifier),
        }
    }

    pub fn verify(&self, proof: &ChainProof) -> Result<(), ChainProofError> {
        let receipt: Receipt = bincode::deserialize(&proof.proof)?;
        self.zk_verifier.verify(&receipt, self.chain_guest_id)?;
        let (proven_root, elf_id): (B256, Digest) = receipt.journal.decode()?;
        let root_hash = proof.block_trie.hash_slow();
        if elf_id != self.chain_guest_id {
            Err(ChainProofError::ElfId {
                expected: self.chain_guest_id,
                got: elf_id,
            })
        } else if proven_root != root_hash {
            Err(ChainProofError::RootHash {
                proven: proven_root,
                actual: root_hash,
            })
        } else {
            Ok(())
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GuestInputError {
    #[error("Chain proof error: {0}")]
    ChainProof(#[from] ChainProofError),
    #[error("Chain client error: {0}")]
    ChainClient(#[from] chain_client::Error),
    #[error("Block not found in chain proof trie: {block_num}")]
    BlockNotFound { block_num: BlockNumber },
    #[error("Block hash mismatch: block_num={block_num}, hash_in_input={hash_in_input}, proven_hash={proven_hash}")]
    BlockHash {
        block_num: BlockNumber,
        hash_in_input: B256,
        proven_hash: B256,
    },
}

pub async fn verify_guest_input(
    chain_client: impl chain_client::Client,
    verifier: impl ChainProofVerifier,
    input: &MultiEvmInput,
) -> Result<(), GuestInputError> {
    for (chain_id, blocks) in input.blocks_by_chain() {
        let block_numbers = blocks.iter().map(|(block_num, _)| *block_num).collect();
        let chain_proof = chain_client
            .get_chain_proof(chain_id, block_numbers)
            .await?;
        verifier.verify(&chain_proof)?;
        for (block_num, block_hash) in blocks {
            let trie_block_hash = chain_proof
                .block_trie
                .get(block_num)
                .ok_or_else(|| GuestInputError::BlockNotFound { block_num })?;
            if trie_block_hash != block_hash {
                return Err(GuestInputError::BlockHash {
                    block_num,
                    hash_in_input: block_hash,
                    proven_hash: trie_block_hash,
                });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use block_header::{BlockHeader, EthBlockHeader, EvmBlockHeader};
    use block_trie::BlockTrie;
    use risc0_zkvm::{serde::to_vec, FakeReceipt, InnerReceipt, ReceiptClaim};
    use traits::Hashable;

    use super::*;

    const CHAIN_GUEST_ID: Digest = Digest::new([0, 0, 0, 0, 0, 0, 0, 1]);

    fn mock_block_header(number: BlockNumber, parent_hash: B256) -> Box<dyn EvmBlockHeader> {
        let header = EthBlockHeader {
            number,
            parent_hash,
            ..Default::default()
        };
        BlockHeader::Eth(header).into()
    }

    fn mock_block_headers(blocks: RangeInclusive<BlockNumber>) -> Vec<Box<dyn EvmBlockHeader>> {
        let mut headers = vec![];
        let mut parent_hash = B256::default();
        for number in blocks {
            let header = mock_block_header(number, parent_hash);
            parent_hash = header.hash_slow();
            headers.push(header);
        }
        headers
    }

    fn mock_block_trie(blocks: RangeInclusive<BlockNumber>) -> BlockTrie {
        let mut trie = BlockTrie::default();
        for header in mock_block_headers(blocks) {
            trie.insert_unchecked(header.number(), &header.hash_slow())
                .unwrap();
        }
        trie
    }

    mod chain_proof {
        use super::*;

        const INVALID_ROOT_HASH: B256 = B256::ZERO;
        const INVALID_ELF_ID: Digest = Digest::ZERO;

        fn mock_receipt(journal: Vec<u8>) -> Receipt {
            let inner: FakeReceipt<ReceiptClaim> =
                FakeReceipt::<ReceiptClaim>::new(ReceiptClaim::ok(CHAIN_GUEST_ID, journal.clone()));

            Receipt::new(InnerReceipt::Fake(inner), journal)
        }

        fn mock_chain_proof(block_trie: BlockTrie, journal: Vec<u8>) -> ChainProof {
            let proof = bincode::serialize(&mock_receipt(journal)).unwrap().into();
            ChainProof { proof, block_trie }
        }

        fn mock_journal(root_hash: B256, elf_id: Digest) -> Vec<u8> {
            let journal = to_vec(&(root_hash, elf_id)).unwrap();
            bytemuck::cast_slice(&journal).into()
        }

        const fn proof_ok(_: &Receipt, _: Digest) -> Result<(), VerificationError> {
            Ok(())
        }

        const fn proof_invalid(_: &Receipt, _: Digest) -> Result<(), VerificationError> {
            Err(VerificationError::InvalidProof)
        }

        #[test]
        fn ok() {
            let verifier = ZkChainProofVerifier::new(CHAIN_GUEST_ID, proof_ok);
            let block_trie = mock_block_trie(0..=1);
            let journal = mock_journal(block_trie.hash_slow(), CHAIN_GUEST_ID);
            let proof = mock_chain_proof(block_trie, journal);
            verifier.verify(&proof).expect("verfication should succeed");
        }

        #[test]
        fn zk_verification_fail() {
            let verifier = ZkChainProofVerifier::new(CHAIN_GUEST_ID, proof_invalid);
            #[allow(clippy::reversed_empty_ranges)]
            let block_trie = mock_block_trie(1..=0);
            let proof = mock_chain_proof(block_trie, vec![]);
            let res = verifier.verify(&proof);
            assert!(matches!(
                res.unwrap_err(),
                ChainProofError::Zk(VerificationError::InvalidProof)
            ));
        }

        #[test]
        fn invalid_root_hash() {
            let verifier = ZkChainProofVerifier::new(CHAIN_GUEST_ID, proof_ok);
            let block_trie = mock_block_trie(0..=1);
            let _root_hash = block_trie.hash_slow();
            let journal = mock_journal(INVALID_ROOT_HASH, CHAIN_GUEST_ID);
            let proof = mock_chain_proof(block_trie, journal);
            let res = verifier.verify(&proof);
            assert!(matches!(
                res.unwrap_err(),
                ChainProofError::RootHash {
                    proven: INVALID_ROOT_HASH,
                    actual: _root_hash,
                }
            ));
        }

        #[test]
        fn invalid_elf_id() {
            let verifier = ZkChainProofVerifier::new(CHAIN_GUEST_ID, proof_ok);
            let block_trie = mock_block_trie(0..=1);
            let journal = mock_journal(block_trie.hash_slow(), INVALID_ELF_ID);
            let proof = mock_chain_proof(block_trie, journal);
            let res = verifier.verify(&proof);
            assert!(matches!(
                res.unwrap_err(),
                ChainProofError::ElfId {
                    expected: CHAIN_GUEST_ID,
                    got: INVALID_ELF_ID
                }
            ));
        }
    }

    mod guest_input {
        use alloy_primitives::ChainId;
        use chain_client::CachedClient;

        use super::*;
        use crate::evm::{env::location::ExecutionLocation, input::EvmInput};

        const CHAIN_ID: ChainId = 1;
        const INVALID_BLOCK_HASH: B256 = B256::ZERO;

        fn mock_chain_client(
            cache: Vec<(ChainId, (Vec<BlockNumber>, ChainProof))>,
        ) -> impl chain_client::Client {
            let cache = cache.into_iter().collect();
            CachedClient::new(cache)
        }

        fn mock_chain_proof(block_trie: BlockTrie) -> ChainProof {
            ChainProof {
                proof: Default::default(),
                block_trie,
            }
        }

        fn mock_evm_input(header: Box<dyn EvmBlockHeader>) -> EvmInput {
            EvmInput {
                header,
                state_trie: Default::default(),
                storage_tries: Default::default(),
                contracts: Default::default(),
                ancestors: Default::default(),
            }
        }

        fn mock_multi_evm_input(blocks: RangeInclusive<BlockNumber>) -> MultiEvmInput {
            let headers = mock_block_headers(blocks);
            let inputs = headers
                .into_iter()
                .map(|header| {
                    (ExecutionLocation::new(header.number(), CHAIN_ID), mock_evm_input(header))
                })
                .collect();
            MultiEvmInput { inputs }
        }

        const fn proof_ok(_: &ChainProof) -> Result<(), ChainProofError> {
            Ok(())
        }

        const fn proof_invalid(_: &ChainProof) -> Result<(), ChainProofError> {
            Err(ChainProofError::Zk(VerificationError::InvalidProof))
        }

        #[tokio::test]
        async fn ok() {
            let block_trie = mock_block_trie(0..=0);
            let chain_proof = mock_chain_proof(block_trie);
            let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0], chain_proof))]);
            let input = mock_multi_evm_input(0..=0);

            verify_guest_input(chain_client, proof_ok, &input)
                .await
                .expect("verfification should succeed");
        }

        #[tokio::test]
        async fn chain_proof_missing() {
            let chain_client = mock_chain_client(vec![]);
            let input = mock_multi_evm_input(0..=0);

            let res = verify_guest_input(chain_client, proof_ok, &input).await;
            assert!(matches!(res.unwrap_err(), GuestInputError::ChainClient(..)));
        }

        #[tokio::test]
        async fn chain_proof_invalid() {
            let block_trie = mock_block_trie(0..=0);
            let chain_proof = mock_chain_proof(block_trie);
            let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0], chain_proof))]);
            let input = mock_multi_evm_input(0..=0);

            let res = verify_guest_input(chain_client, proof_invalid, &input).await;
            assert!(matches!(res.unwrap_err(), GuestInputError::ChainProof(..)));
        }

        #[tokio::test]
        async fn block_not_in_trie() {
            let chain_proof = mock_chain_proof(BlockTrie::default());
            let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0], chain_proof))]);
            let input = mock_multi_evm_input(0..=0);

            let res = verify_guest_input(chain_client, proof_ok, &input).await;
            assert!(matches!(res.unwrap_err(), GuestInputError::BlockNotFound { block_num: 0 }));
        }

        #[tokio::test]
        async fn block_hash_mismatch() {
            let block_header = mock_block_header(0, Default::default());
            let _block_hash = block_header.hash_slow();
            let mut block_trie = BlockTrie::default();
            block_trie.insert_unchecked(0, &INVALID_BLOCK_HASH).unwrap();
            let chain_proof = mock_chain_proof(block_trie);
            let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0], chain_proof))]);
            let input = mock_multi_evm_input(0..=0);

            let res = verify_guest_input(chain_client, proof_ok, &input).await;
            assert!(matches!(
                res.unwrap_err(),
                GuestInputError::BlockHash {
                    block_num: 0,
                    hash_in_input: _block_hash,
                    proven_hash: INVALID_BLOCK_HASH
                }
            ));
        }
    }
}
