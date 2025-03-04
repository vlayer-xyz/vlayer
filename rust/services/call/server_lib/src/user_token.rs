use axum_extra::headers::{authorization::Bearer, Authorization};
use derive_more::{Deref, From};

#[derive(From, Clone, Debug, Deref)]
pub struct Token(String);

impl From<Authorization<Bearer>> for Token {
    fn from(value: Authorization<Bearer>) -> Self {
        Self(value.token().into())
    }
}
