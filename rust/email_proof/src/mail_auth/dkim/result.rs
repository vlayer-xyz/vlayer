#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Result {
    Pass,
    Neutral(crate::mail_auth::Error),
    Fail(crate::mail_auth::Error),
    PermError(crate::mail_auth::Error),
    TempError(crate::mail_auth::Error),
    None,
}
