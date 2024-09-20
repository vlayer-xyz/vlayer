use alloy_primitives::FixedBytes;
use risc0_zkvm::{
    sha::Digestible,
    FakeReceipt, Groth16Receipt,
    InnerReceipt::{self, Fake, Groth16},
    Receipt, ReceiptClaim,
};

use crate::host::error::HostError;
use call_engine::{ProofMode, Seal};

const VERIFIER_SELECTOR_LENGTH: usize = 4;
const GROTH16_PROOF_SIZE: usize = 256;
const SEAL_BYTES_SIZE: usize = GROTH16_PROOF_SIZE;

const FAKE_VERIFIER_SELECTOR: VerifierSelector = VerifierSelector([0xde, 0xaf, 0xbe, 0xef]); // Should align with constant in FakeProofVerifier.sol

type SealBytesT = [u8; SEAL_BYTES_SIZE];

#[derive(Debug, Default, PartialEq, Eq)]
struct VerifierSelector([u8; VERIFIER_SELECTOR_LENGTH]);

#[derive(Clone)]
pub(crate) struct EncodableReceipt(InnerReceipt);

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

    fn verifier_selector(&self) -> Option<VerifierSelector> {
        match &self.0 {
            Groth16(inner) => Some(Self::extract_groth16_verifier_selector(inner)),
            Fake(_) => Some(FAKE_VERIFIER_SELECTOR),
            _ => None,
        }
    }

    fn extract_fake_seal(inner: &FakeReceipt<ReceiptClaim>) -> Option<SealBytesT> {
        let mut result: SealBytesT = [0; SEAL_BYTES_SIZE];
        let mut seal: Vec<u8> = inner.claim.digest().as_bytes().into();
        seal.resize(SEAL_BYTES_SIZE, 0);
        result.clone_from_slice(seal.as_slice());

        Some(result)
    }

    fn extract_groth16_seal(inner: &Groth16Receipt<ReceiptClaim>) -> Option<SealBytesT> {
        let mut result: SealBytesT = [0; SEAL_BYTES_SIZE];
        let bytes = &inner.seal;

        if bytes.len() != GROTH16_PROOF_SIZE {
            return None;
        }

        result.clone_from_slice(bytes.as_slice());

        Some(result)
    }

    fn extract_groth16_verifier_selector(inner: &Groth16Receipt<ReceiptClaim>) -> VerifierSelector {
        let mut selector: VerifierSelector = Default::default();
        let selector_bytes = &inner.verifier_parameters.as_bytes()[..VERIFIER_SELECTOR_LENGTH];
        selector.0.clone_from_slice(selector_bytes);

        selector
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
            .ok_or(HostError::SealEncodingError("Could not retreive valid seal bytes".into()))
            .map(split_seal_into_bytes)?;

        let verifier_selector: FixedBytes<VERIFIER_SELECTOR_LENGTH> = value
            .verifier_selector()
            .ok_or(HostError::SealEncodingError("Could not retreive verifier selector".into()))
            .map(|sel| sel.0.into())?;

        Ok(Seal {
            verifierSelector: verifier_selector,
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
    use alloy_sol_types::SolValue;
    use risc0_zkvm::sha::Digestible;
    use risc0_zkvm::{Groth16Receipt, Groth16ReceiptVerifierParameters, ReceiptClaim};

    const MOCK_CALL_GUEST_ID: [u8; 32] = [1; 32];

    const ETH_WORD_SIZE: usize = 32;
    const SEAL_ENCODING_SIZE: usize = ETH_WORD_SIZE + GROTH16_PROOF_SIZE + ETH_WORD_SIZE;

    const GROTH16_MOCK_SEAL: [u8; GROTH16_PROOF_SIZE] = [1; GROTH16_PROOF_SIZE];

    // stable, expected selector by solidity groth16 verifiers
    // must be keept in sync with value from `contracts/test/helpers/Groth16VerifierSelector.sol`
    const GROTH16_VERIFIER_SELECTOR: VerifierSelector = VerifierSelector([0x31, 0x0f, 0xe5, 0x98]);

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
            ReceiptClaim::ok(MOCK_CALL_GUEST_ID, journal.clone()).into(),
            Groth16ReceiptVerifierParameters::default().digest(),
        );

        Receipt::new(Groth16(inner), journal)
    }

    fn mock_fake_receipt() -> Receipt {
        let journal = mock_journal();
        let inner: FakeReceipt<ReceiptClaim> =
            FakeReceipt::<ReceiptClaim>::new(ReceiptClaim::ok(MOCK_CALL_GUEST_ID, journal.clone()));

        Receipt::new(Fake(inner), journal)
    }

    fn mock_other_receipt() -> Receipt {
        use risc0_zkvm::CompositeReceipt;
        use std::{mem::MaybeUninit, ptr::addr_of_mut};

        let mut uninit: std::mem::MaybeUninit<CompositeReceipt> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();
        unsafe {
            // done just to fool the compiler, since CompositeReceipt is non-exhaustive
            addr_of_mut!((*ptr).segments).write(Default::default());
            addr_of_mut!((*ptr).assumption_receipts).write(Default::default());
            addr_of_mut!((*ptr).verifier_parameters).write(Default::default());
        }
        let receipt = unsafe { uninit.assume_init() };

        Receipt::new(InnerReceipt::Composite(receipt), mock_journal())
    }

    mod abi_encoding {

        use super::*;

        #[test]
        fn expected_encoding_size() {
            use alloy_sol_types::SolType;
            assert_eq!(SEAL_ENCODING_SIZE, Seal::ENCODED_SIZE.unwrap())
        }
    }

    mod encodable_receipt {
        use super::*;

        mod try_into_seal {
            use super::*;

            #[test]
            fn seal_has_proof_mode() {
                let receipt: EncodableReceipt = mock_fake_receipt().into();
                let seal: Seal = receipt.try_into().unwrap();

                assert_eq!(ProofMode::FAKE, seal.mode);
            }

            #[test]
            fn seal_has_verifier_selector() {
                let receipt: EncodableReceipt = mock_fake_receipt().into();
                let seal: Seal = receipt.try_into().unwrap();

                assert_eq!(&FAKE_VERIFIER_SELECTOR.0, seal.verifierSelector.as_slice());
            }

            #[test]
            fn seal_has_seal_bytes() {
                let receipt: EncodableReceipt = mock_fake_receipt().into();
                let seal: Seal = receipt.clone().try_into().unwrap();

                let seal_bytes: [[u8; 32]; 8] = seal.seal.map(Into::into);
                let seal_bytes = seal_bytes.concat();

                assert_eq!(receipt.seal_bytes().unwrap(), seal_bytes.as_slice());
            }
        }

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

            #[test]
            fn returns_none_for_other_receipt() {
                let receipt: EncodableReceipt = mock_other_receipt().into();
                assert_eq!(None, receipt.proof_mode());
            }
        }

        mod verifier_selector {
            use super::*;

            #[test]
            fn returns_fake_verifier_selector_for_fake_receipt() {
                let receipt: EncodableReceipt = mock_fake_receipt().into();
                assert_eq!(FAKE_VERIFIER_SELECTOR, receipt.verifier_selector().unwrap())
            }

            #[test]
            fn returns_groth16_verifier_params_for_groth16_receipt() {
                let receipt: EncodableReceipt = mock_groth16_receipt().into();
                assert_eq!(GROTH16_VERIFIER_SELECTOR, receipt.verifier_selector().unwrap())
            }

            #[test]
            fn returns_none_for_other_receipt() {
                let receipt: EncodableReceipt = mock_other_receipt().into();
                assert_eq!(None, receipt.verifier_selector());
            }
        }
        mod seal_bytes {
            use super::*;

            mod fake_proof_receipt {
                use super::*;

                #[test]
                fn fake_seal_bytes_starts_with_claim_digest() {
                    let journal = mock_journal();
                    let claim = ReceiptClaim::ok(MOCK_CALL_GUEST_ID, journal);

                    let receipt: EncodableReceipt = mock_fake_receipt().into();
                    let first_word = &receipt.seal_bytes().unwrap()[..ETH_WORD_SIZE];

                    assert_eq!(claim.digest().as_bytes(), first_word);
                }

                #[test]
                fn other_bytes_are_zeroed() {
                    let receipt: EncodableReceipt = mock_fake_receipt().into();
                    let other_words = &receipt.seal_bytes().unwrap()[ETH_WORD_SIZE..];

                    assert_eq!(&[0u8; SEAL_BYTES_SIZE - ETH_WORD_SIZE], other_words);
                }
            }
            mod groth16_proof_receipt {
                use super::*;

                #[test]
                fn returns_seal_for_groth16() {
                    let receipt: EncodableReceipt = mock_groth16_receipt().into();
                    let expected_seal_bytes = &receipt.0.groth16().unwrap().seal;

                    assert_eq!(expected_seal_bytes.as_slice(), receipt.seal_bytes().unwrap());
                }
                #[test]
                fn returns_none_for_invalid_groth16_seal_size() {
                    let mut seal_bytes: Vec<u8> = GROTH16_MOCK_SEAL.into();
                    seal_bytes.push(1);

                    let inner = Groth16Receipt::<ReceiptClaim>::new(
                        seal_bytes,
                        ReceiptClaim::ok(MOCK_CALL_GUEST_ID, mock_journal()).into(),
                        Groth16ReceiptVerifierParameters::default().digest(),
                    );
                    let receipt: EncodableReceipt =
                        Receipt::new(Groth16(inner), mock_journal()).into();

                    assert_eq!(None, receipt.seal_bytes());
                }
            }
        }
    }
}
