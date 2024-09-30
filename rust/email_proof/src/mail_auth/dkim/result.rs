use crate::mail_auth::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Result {
    Pass,
    Neutral(Error),
    Fail(Error),
    PermError(Error),
    TempError(Error),
    None,
}
