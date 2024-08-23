use alloy_primitives::FixedBytes;
use risc0_zkvm::{
    sha::Digestible,
    FakeReceipt, Groth16Receipt,
    InnerReceipt::{self, Fake, Groth16},
    Receipt, ReceiptClaim,
};

use crate::host::error::HostError;
use call_engine::{ProofMode, Seal};

const GROTH16_PROOF_SIZE: usize = 256;
const SEAL_BYTES_SIZE: usize = GROTH16_PROOF_SIZE;

const FAKE_VERIFIER_SELECTOR: [u8; 4] = [0xde, 0xaf, 0xbe, 0xef]; // Should align with constant in FakeProofVerifier.sol

type SealBytesT = [u8; SEAL_BYTES_SIZE];

pub struct EncodableReceipt(InnerReceipt);

impl EncodableReceipt {
    fn proof_mode(&self) -> Option<ProofMode> {
        match self.0 {
            Groth16(_) => Some(ProofMode::GROTH16),
            Fake(_) => Some(ProofMode::FAKE),
            _ => None,
        }
    }

    fn seal_bytes(&self) -> Option<SealBytesT> {
        match &self.0 {
            Groth16(inner) => Self::extract_groth16_seal(inner),
            Fake(inner) => Self::extract_fake_seal(inner),
            _ => None,
        }
    }

    fn extract_fake_seal(inner: &FakeReceipt<ReceiptClaim>) -> Option<SealBytesT> {
        let mut result = [0; GROTH16_PROOF_SIZE];
        let mut seal_suffix: Vec<u8> = inner.claim.digest().as_bytes().into();

        let mut seal: Vec<u8> = FAKE_VERIFIER_SELECTOR.to_vec();
        seal.append(&mut seal_suffix);

        seal.resize(GROTH16_PROOF_SIZE, 0);
        result.clone_from_slice(seal.as_slice());

        Some(result)
    }

    fn extract_groth16_seal(inner: &Groth16Receipt<ReceiptClaim>) -> Option<SealBytesT> {
        let mut result = [0; GROTH16_PROOF_SIZE];
        let bytes = &inner.seal;

        if bytes.len() != GROTH16_PROOF_SIZE {
            return None;
        }

        result.clone_from_slice(bytes.as_slice());

        Some(result)
    }
}

impl From<Receipt> for EncodableReceipt {
    fn from(value: Receipt) -> Self {
        Self(value.inner)
    }
}

impl TryFrom<EncodableReceipt> for Seal {
    type Error = HostError;

    fn try_from(value: EncodableReceipt) -> Result<Self, Self::Error> {
        let seal_type = value
            .proof_mode()
            .ok_or(HostError::SealEncodingError("Invalid proof type".into()))?;

        let raw_seal = value
            .seal_bytes()
            .ok_or(HostError::SealEncodingError(
                "Could not retreive valid seal bytes".into(),
            ))
            .map(split_seal_into_bytes)?;

        Ok(Seal {
            seal: raw_seal,
            mode: seal_type,
        })
    }
}

fn split_seal_into_bytes(bytes: SealBytesT) -> [FixedBytes<32>; 8] {
    let chunks: Vec<FixedBytes<32>> = bytes
        .chunks(32)
        .map(|chunk: &[u8]| {
            let mut word: [u8; 32] = [0; 32];
            word.clone_from_slice(chunk);
            FixedBytes::<32>::new(word)
        })
        .collect();

    let mut result: [FixedBytes<32>; 8] = Default::default();
    result.clone_from_slice(chunks.as_slice());

    result
}

#[cfg(test)]
mod test {
    use super::*;

    use alloy_primitives::hex::FromHex;
    use alloy_primitives::{Address, Uint};
    use alloy_sol_types::{SolType, SolValue};
    use call_guest_wrapper::RISC0_CALL_GUEST_ID;

    use risc0_zkvm::sha::Digestible;
    use risc0_zkvm::{Groth16Receipt, Groth16ReceiptVerifierParameters, ReceiptClaim};

    const ETH_WORD_SIZE: usize = 32;
    const SEAL_ENCODING_SIZE: usize = GROTH16_PROOF_SIZE + ETH_WORD_SIZE;

    const GROTH16_MOCK_SEAL: [u8; GROTH16_PROOF_SIZE] = [1; GROTH16_PROOF_SIZE];

    fn mock_journal() -> Vec<u8> {
        let execution_commitment = call_engine::ExecutionCommitment {
            proverContractAddress: Address::from_hex("0x0000000000000000000000000000000000000001")
                .unwrap(),
            functionSelector: FixedBytes::new([1, 2, 3, 4]),
            settleBlockNumber: Uint::<256, 4>::from(1),
            settleBlockHash: FixedBytes::new([0; 32]),
        };
        execution_commitment.abi_encode()
    }

    fn mock_groth16_receipt() -> Receipt {
        let journal = mock_journal();
        let inner = Groth16Receipt::<ReceiptClaim>::new(
            GROTH16_MOCK_SEAL.into(),
            ReceiptClaim::ok(RISC0_CALL_GUEST_ID, journal.clone()).into(),
            Groth16ReceiptVerifierParameters::default().digest(),
        );

        Receipt::new(Groth16(inner), journal)
    }

    fn mock_fake_receipt() -> Receipt {
        let journal = mock_journal();
        let inner: FakeReceipt<ReceiptClaim> = FakeReceipt::<ReceiptClaim>::new(ReceiptClaim::ok(
            RISC0_CALL_GUEST_ID,
            journal.clone(),
        ));

        Receipt::new(Fake(inner), journal)
    }

    #[test]
    fn expected_encoding_size() {
        assert_eq!(SEAL_ENCODING_SIZE, Seal::ENCODED_SIZE.unwrap())
    }

    #[test]
    fn can_encode_seal_into_abi() {
        let receipt: EncodableReceipt = mock_groth16_receipt().into();
        let seal: Seal = receipt.try_into().unwrap();

        let mut expected_encoding = vec![1; 256];

        expected_encoding.extend_from_slice(ProofMode::GROTH16.abi_encode().as_slice());

        assert_eq!(expected_encoding, seal.abi_encode().as_slice());
    }

    #[test]
    fn can_encode_fake_seal_into_abi() {
        let receipt: EncodableReceipt = mock_fake_receipt().into();
        let seal: Seal = receipt.try_into().unwrap();

        let claim: ReceiptClaim = ReceiptClaim::ok(RISC0_CALL_GUEST_ID, mock_journal());
        let mut expected_suffix: Vec<u8> = claim.digest().as_bytes().into();

        let mut expected_encoding = FAKE_VERIFIER_SELECTOR.to_vec();

        expected_encoding.append(&mut expected_suffix);
        expected_encoding.resize(256, 0);
        expected_encoding.extend_from_slice(ProofMode::FAKE.abi_encode().as_slice());

        assert_eq!(expected_encoding, seal.abi_encode().as_slice());
    }

    #[test]
    fn seal_encodes_proof_mode() {
        let receipt: EncodableReceipt = mock_fake_receipt().into();
        let seal: Seal = receipt.try_into().unwrap();

        assert_eq!(ProofMode::FAKE, seal.mode);
    }

    mod encodable_receipt {
        use super::*;

        mod proof_mode {
            use super::*;
            #[test]
            fn returns_groth16_mode_for_groth16_receipt() {
                let receipt: EncodableReceipt = mock_groth16_receipt().into();

                assert_eq!(ProofMode::GROTH16, receipt.proof_mode().unwrap())
            }

            #[test]
            fn returns_fake_mode_for_fake_receipt() {
                let receipt: EncodableReceipt = mock_fake_receipt().into();
                assert_eq!(ProofMode::FAKE, receipt.proof_mode().unwrap())
            }
        }
    }
}
