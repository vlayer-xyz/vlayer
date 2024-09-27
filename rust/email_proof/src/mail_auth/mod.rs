#![allow(dead_code)]

use authenticated_message::AuthenticatedMessage;
use error::Error;
use resolver::Resolver;

pub(crate) mod authenticated_message;
pub mod common;
pub mod dkim;
pub(crate) mod error;
pub(crate) mod resolver;

pub type Result<T> = std::result::Result<T, Error>;
