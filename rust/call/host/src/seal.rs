use risc0_zkvm::{
    FakeReceipt, Groth16Receipt,
    InnerReceipt::{self, Fake, Groth16},
    Receipt, ReceiptClaim,
};

use call_engine::Seal as SealSol;

use crate::host::error::HostError;

const SEAL_SIZE: usize = 36;
const HALF_SEAL_SIZE: usize = SEAL_SIZE / 2;
const SEAL_TYPE_SIZE: usize = 1;

pub struct EncodableReceipt(InnerReceipt);

#[repr(u8)]
enum ProofMode {
    Groth16 = 0,
    Fake = 1,
}

impl From<Receipt> for EncodableReceipt {
    fn from(value: Receipt) -> Self {
        Self(value.inner)
    }
}

impl TryFrom<EncodableReceipt> for SealSol {
    type Error = HostError;

    fn try_from(value: EncodableReceipt) -> Result<Self, Self::Error> {
        let seal_type =
            proof_mode(&value).ok_or(HostError::SealEncodingError("Invalid proof type".into()))?;
        let (lhv, rhv): ([u8; HALF_SEAL_SIZE], [u8; HALF_SEAL_SIZE]) = extract_raw_seal(&value)
            .and_then(split_raw_seal)
            .ok_or(HostError::SealEncodingError("Invalid seal length".into()))?;

        let mut rhv_with_seal_type: [u8; HALF_SEAL_SIZE + SEAL_TYPE_SIZE] = Default::default();

        rhv_with_seal_type[..HALF_SEAL_SIZE].copy_from_slice(&rhv);
        rhv_with_seal_type[HALF_SEAL_SIZE] = seal_type as u8;

        Ok(SealSol {
            lhv: lhv.into(),
            rhv: rhv_with_seal_type.into(),
        })
    }
}

fn extract_raw_seal(value: &EncodableReceipt) -> Option<Vec<u8>> {
    match &value.0 {
        Groth16(inner) => Some(from_groth16_receipt(inner)),
        Fake(inner) => Some(from_fake_receipt(inner)),
        _ => None,
    }
}

fn proof_mode(value: &EncodableReceipt) -> Option<ProofMode> {
    match &value.0 {
        Groth16(_) => Some(ProofMode::Groth16),
        Fake(_) => Some(ProofMode::Fake),
        _ => None,
    }
}

fn from_groth16_receipt(receipt: &Groth16Receipt<ReceiptClaim>) -> Vec<u8> {
    receipt.seal.clone()
}
fn from_fake_receipt(_: &FakeReceipt<ReceiptClaim>) -> Vec<u8> {
    vec![0; SEAL_SIZE]
}

fn split_raw_seal(raw_seal: Vec<u8>) -> Option<([u8; HALF_SEAL_SIZE], [u8; HALF_SEAL_SIZE])> {
    if raw_seal.len() != SEAL_SIZE {
        return None;
    }

    let mut lhv: [u8; HALF_SEAL_SIZE] = Default::default();
    let mut rhv: [u8; HALF_SEAL_SIZE] = Default::default();

    let (l, r): (&[u8; HALF_SEAL_SIZE], &[u8]) = raw_seal.split_first_chunk()?;
    lhv.clone_from_slice(l);
    rhv.clone_from_slice(r);

    Some((lhv, rhv))
}

#[cfg(test)]
mod test {
    use super::*;

    use alloy_sol_types::{SolType, SolValue};
    use call_guest_wrapper::RISC0_CALL_GUEST_ID;

    use risc0_zkvm::sha::Digestible;
    use risc0_zkvm::{Groth16Receipt, Groth16ReceiptVerifierParameters, ReceiptClaim};

    const ETH_WORD_SIZE: usize = 32;
    const SEAL_ENCODING_SIZE: usize = 2 * ETH_WORD_SIZE;

    const INNER_SEAL: [u8; SEAL_SIZE] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
    ];

    fn mock_journal() -> Vec<u8> {
        Vec::<u8>::default()
    }

    fn mock_groth16_receipt() -> Receipt {
        let journal = mock_journal();
        let inner = Groth16Receipt::<ReceiptClaim>::new(
            INNER_SEAL.into(),
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
        assert_eq!(SEAL_ENCODING_SIZE, SealSol::ENCODED_SIZE.unwrap())
    }

    #[test]
    fn can_encode_seal_into_abi() {
        let receipt: EncodableReceipt = mock_groth16_receipt().into();
        let seal: SealSol = receipt.try_into().unwrap();

        // ABI  ENCODED SEAL structure
        // |      SEAL LHV       |      PADDING     |      SEAL RHV        |MODE|     PADDING     |
        // |0..................17|18..............31|32..................49| 50 |51.............63|

        let expected_encoding: [u8; SEAL_ENCODING_SIZE] = [
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, // LHV
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PADDING
            18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, // RHV
            0,  // PROOF MODE
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PADDING
        ];

        assert_eq!(expected_encoding, seal.abi_encode().as_slice());
    }

    #[test]
    fn receipt_into_seal_conversion_splits_inner_seal() {
        let receipt: EncodableReceipt = mock_groth16_receipt().into();
        let seal: SealSol = receipt.try_into().unwrap();

        let lhv = seal.lhv.0;
        let rhv = &(seal.rhv.0[..HALF_SEAL_SIZE]);

        assert_eq!(&INNER_SEAL[0..HALF_SEAL_SIZE], lhv);
        assert_eq!(&INNER_SEAL[HALF_SEAL_SIZE..], rhv);
    }

    #[test]
    fn seal_encodes_proof_mode() {
        let groth16_receipt: EncodableReceipt = mock_groth16_receipt().into();
        let groth16_seal: SealSol = groth16_receipt.try_into().unwrap();

        let fake_receipt: EncodableReceipt = mock_fake_receipt().into();
        let fake_seal: SealSol = fake_receipt.try_into().unwrap();

        assert_eq!(ProofMode::Groth16 as u8, groth16_seal.rhv.0[HALF_SEAL_SIZE]);
        assert_eq!(ProofMode::Fake as u8, fake_seal.rhv.0[HALF_SEAL_SIZE]);
    }

    mod split_raw_seal {

        use super::*;

        #[test]
        fn splits_seal_of_expected_size() {
            let (lhv, rhv) = split_raw_seal(INNER_SEAL.into()).unwrap();

            assert_eq!(&INNER_SEAL[0..HALF_SEAL_SIZE], lhv);
            assert_eq!(&INNER_SEAL[HALF_SEAL_SIZE..SEAL_SIZE], rhv);
        }

        #[test]
        fn returns_none_for_invalid_seal_size() {
            let none = split_raw_seal(INNER_SEAL[1..].into());
            assert!(none.is_none())
        }
    }
}
