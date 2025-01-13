pub mod verifier;
pub mod web_proof;

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures;

mod errors;
mod http_transaction_parser;
mod request_transcript;
mod response_transcript;
mod web;
