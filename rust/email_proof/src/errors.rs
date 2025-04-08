use derivative::Derivative;
use thiserror::Error;

#[derive(Error, Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum Error {
    #[error("Could not parse email: {0}")]
    EmailParse(
        #[from]
        #[derivative(PartialEq = "ignore")]
        mailparse::MailParseError,
    ),
    #[error("Invalid UnverifiedEmail calldata: {0}")]
    Calldata(#[from] alloy_sol_types::Error),
    #[error("Error verifying DKIM: {0}")]
    DkimVerification(#[from] cfdkim::DKIMError),
    #[error("Domain mismatch: expected {0}, actual {1}")]
    DomainMismatch(String, String),
    #[error("Invalid DKIM public key record: {0}")]
    InvalidDkimRecord(String),
    #[error("Invalid From header: {0}")]
    InvalidFromHeader(String),
    #[error("Invalid newline separator: lone '\\n' found not preceded by '\\r'. Found byte {0}")]
    LoneNewLine(u8),
    #[error("Missing CRLF-CRLF separator between email headers and body")]
    MissingBodySeparator,
    #[error("Missing From header")]
    MissingFromHeader,
    #[error("Missing required header `{0}` in DKIM h= tag")]
    MissingRequiredHeaderTag(String),
    #[error("No matching DKIM header found for From domain `{0}`")]
    NoDkimMatchingFromDomain(String),
    #[error("No From header found")]
    NoFromHeader,
    #[error("Expected exactly one DKIM-Signature header, found {0}")]
    InvalidDkimHeaderCount(usize),
    #[error("VDNS signature verification failed: {0}")]
    VdnsSignatureVerification(
        #[from]
        #[derivative(PartialEq = "ignore")]
        verifiable_dns::RecordVerifierError,
    ),
}
