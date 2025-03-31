use derivative::Derivative;
use thiserror::Error;

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq, Eq)]
#[error(transparent)]
pub struct Error(
    #[from]
    #[derivative(PartialEq = "ignore")]
    anyhow::Error,
);
pub type Result<T> = std::result::Result<T, Error>;
