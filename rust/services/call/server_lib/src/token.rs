use derive_more::{Deref, From};
use derive_new::new;
use serde::Deserialize;

#[derive(new, From, Clone, Debug, Deref, Deserialize)]
pub struct Token(String);
