#![allow(dead_code)]

pub(crate) mod authenticated_message;
pub mod common;
pub mod dkim;
pub(crate) mod error;
pub(crate) mod resolver;

use authenticated_message::AuthenticatedMessage;
use error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;
