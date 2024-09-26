use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not parse email: {0}")]
    EmailParse(mailparse::MailParseError),
    #[error("Invalid UnverifiedEmail calldata: {0}")]
    Calldata(alloy_sol_types::Error),
    #[error("Error verifying DKIM: {0}")]
    DkimVerification(mail_auth::Error),
}
