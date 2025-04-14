use rsa::{
    pkcs1v15,
    pkcs1v15::VerifyingKey,
    pkcs8::{DecodePublicKey, spki},
    sha2::Sha256,
    signature::{self, Verifier},
};

use crate::{
    PublicKey, Signature, common, common::to_payload::ToPayload,
    dns_over_https::types::Record as DNSRecord,
};

#[derive(thiserror::Error, Debug)]
pub enum RecordVerifierError {
    #[error("Public key decoding error: {0}")]
    PublicKeyDecoding(#[from] spki::Error),
    #[error("Signature decoding error: {0}")]
    SignatureDecoding(#[source] signature::Error),
    #[error("Signature verification error")]
    SignatureVerification(#[from] signature::Error),
}

pub fn verify_signature(
    record: &DNSRecord,
    valid_until: u64,
    pub_key: &PublicKey,
    signature: &Signature,
) -> Result<(), RecordVerifierError> {
    let verifying_key = VerifyingKey::<Sha256>::from_public_key_der(&pub_key.0)?;
    let rsa_signature = pkcs1v15::Signature::try_from(signature.0.iter().as_slice())
        .map_err(RecordVerifierError::SignatureDecoding)?;

    verifying_key
        .verify(&common::record::Record::new(record, valid_until).to_payload(), &rsa_signature)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use bytes::Bytes;
    use lazy_static::lazy_static;

    use super::*;
    use crate::{
        VerificationData,
        verifiable_dns::{sign_record::sign_record, signer::Signer},
    };

    lazy_static! {
        static ref RECORD: DNSRecord = DNSRecord {
            name: "google._domainkey.vlayer.xyz".into(),
            record_type: crate::dns_over_https::types::RecordType::TXT,
            ttl: 300,
            data: "Hello".into(),
        };
    }

    #[test]
    fn can_verify_record() {
        let VerificationData {
            signature,
            pub_key,
            valid_until,
        } = sign_record(&Signer::default(), &RECORD, 123);
        verify_signature(&RECORD, valid_until, &pub_key, &signature).unwrap();
    }

    #[test]
    fn fails_on_modified_record() {
        let VerificationData {
            signature,
            pub_key,
            valid_until,
        } = sign_record(&Signer::default(), &RECORD, 123);
        let modified_record = DNSRecord {
            data: "World".into(),
            ..RECORD.clone()
        };
        assert_eq!(
            verify_signature(&modified_record, valid_until, &pub_key, &signature)
                .unwrap_err()
                .to_string(),
            "Signature verification error"
        );
    }

    #[test]
    fn fails_on_invalid_public_key() {
        assert_eq!(
            verify_signature(
                &RECORD,
                100,
                &PublicKey(Bytes::from(vec![0_u8])),
                &Signature(Bytes::from(vec![0_u8]))
            )
            .unwrap_err()
            .to_string(),
            "Public key decoding error: ASN.1 error: unknown/unsupported ASN.1 DER tag: 0x00"
        );
    }
}
