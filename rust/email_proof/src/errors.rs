use std::io::Bytes;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not parse email: {0}")]
    EmailParse(#[from] mailparse::MailParseError),
    #[error("Invalid UnverifiedEmail calldata: {0}")]
    Calldata(#[from] alloy_sol_types::Error),
    #[error("Error verifying DKIM: {0}")]
    DkimVerification(#[from] cfdkim::DKIMError),
    #[error("Domain mismatch: {0} != {1}")]
    DomainMismatch(String, String),
    #[error("Invalid DKIM public key record: {0}")]
    InvalidDkimRecord(String),
    #[error("Invalid From header: {0}")]
    InvalidFromHeader(String),
    #[error("VDNS signature verification failed: {0}")]
    VdnsSignatureVerification(#[from] verifiable_dns::RecordVerifierError),
}
