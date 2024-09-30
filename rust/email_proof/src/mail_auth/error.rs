use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    DkimRecord(String),
    FailedAuidMatch,
    FailedBodyHashMatch,
    SignatureExpired,
    Dns(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SignatureExpired => write!(f, "Signature expired"),
            Error::FailedBodyHashMatch => {
                write!(f, "Calculated body hash does not match signature hash")
            }
            Error::FailedAuidMatch => write!(f, "AUID does not match domain name"),
            Error::Dns(err) => write!(f, "DNS resolution error: {err}"),
            Error::DkimRecord(err) => write!(f, "Failed to parse DKIM record: {err}"),
        }
    }
}
impl std::error::Error for Error {}
