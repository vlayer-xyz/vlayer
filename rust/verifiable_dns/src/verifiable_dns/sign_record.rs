use crate::{
    VerificationData, common,
    dns_over_https::types::Record as DNSRecord,
    verifiable_dns::{signer::Signer, time::Timestamp},
};

pub fn sign_record(
    signer: &Signer,
    record: &DNSRecord,
    valid_until: Timestamp,
) -> VerificationData {
    let signature = signer.sign(&common::record::Record::new(record, valid_until));

    VerificationData {
        signature,
        valid_until,
        pub_key: signer.public_key(),
    }
}
