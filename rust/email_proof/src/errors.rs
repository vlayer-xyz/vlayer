use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not parse email: {0}")]
    EmailParse(mailparse::MailParseError),
    #[error("Invalid UnverifiedEmail calldata: {0}")]
    Calldata(alloy_sol_types::Error),
    #[error("Error verifying DKIM: {0}")]
    DkimVerification(cfdkim::DKIMError),
    #[error("Invalid DKIM public key record: {0}")]
    InvalidDkimRecord(String),
    #[error("Invalid From header: {0}")]
    InvalidFromHeader(String),
}
