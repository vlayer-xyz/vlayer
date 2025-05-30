pub mod email_proof;
mod helpers;
pub mod json;
pub mod precompile;
mod regex;
pub mod system;
pub mod url_pattern;
mod web_proof;

use alloy_primitives::{Address, Bytes};
use email_proof::verify as email_proof;
use helpers::generate_precompile;
use json::{
    get_bool as json_get_bool, get_float_as_int as json_get_float_as_int, get_int as json_get_int,
    get_string as json_get_string,
};
use precompile::{Precompile, Tag, gas_used};
use regex::{capture as regex_capture, is_match as regex_is_match};
use revm::precompile::{
    Precompile as RawPrecompile, PrecompileOutput, PrecompileResult, PrecompileWithAddress,
};
use thiserror::Error;
use url_pattern::test as url_pattern_test;
use web_proof::verify as web_proof;

pub fn precompiles(is_vlayer_test: bool) -> Vec<Precompile> {
    let mut list = vec![
        generate_precompile!(0x00, web_proof, 1000, 10, Tag::WebProof),
        generate_precompile!(0x01, email_proof, 1000, 10, Tag::EmailProof),
        generate_precompile!(0x02, json_get_string, 1000, 10, Tag::JsonGetString),
        generate_precompile!(0x03, json_get_int, 1000, 10, Tag::JsonGetInt),
        generate_precompile!(0x04, json_get_bool, 1000, 10, Tag::JsonGetBool),
        generate_precompile!(0x05, json_get_float_as_int, 1000, 10, Tag::JsonGetFloatAsInt),
        generate_precompile!(0x10, regex_is_match, 1000, 10, Tag::RegexIsMatch),
        generate_precompile!(0x11, regex_capture, 1000, 10, Tag::RegexCapture),
        generate_precompile!(0x20, url_pattern_test, 1000, 10, Tag::UrlPatternTest),
    ];

    if is_vlayer_test {
        list.push(generate_precompile!(0x30, system::is_vlayer_test, 1000, 10, Tag::IsVlayerTest));
    }

    list
}

pub fn precompile_by_address(address: &Address, is_vlayer_test: bool) -> Option<Precompile> {
    precompiles(is_vlayer_test)
        .into_iter()
        .find(|precomp| precomp.address() == address)
}

pub fn precompile_by_tag(tag: &Tag) -> Option<Precompile> {
    precompiles(false)
        .into_iter()
        .find(|precomp| precomp.tag() == *tag)
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("Precompile not allowed for travel calls: {0}")]
pub struct PrecompileNotAllowedError(pub Tag);

/// Returns `true` if the precompile is time-dependent and must not be used in travel calls.
///
/// Specifically, `WebProof` and `EmailProof` rely on real-world data (e.g., DNS records, timestamps)
/// that can be manipulated when paired with Time Travel:
/// - A user may jump to a block where an expired `validUntil` timestamp is still valid.
/// - Or abuse a DNS key that was valid in the past but has since been revoked.
///
/// To preserve integrity, these precompiles are considered unsafe in historical locations.
pub const fn is_time_dependent(precompile: &Precompile) -> bool {
    matches!(precompile.tag(), Tag::WebProof | Tag::EmailProof)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod verify_precompile_allowed_in_travel_call {
        use lazy_static::lazy_static;

        use super::*;

        lazy_static! {
            static ref WEB_PROOF: Precompile = precompile_by_tag(&Tag::WebProof).unwrap();
            static ref EMAIL_PROOF: Precompile = precompile_by_tag(&Tag::EmailProof).unwrap();
            static ref JSON_GET_STRING: Precompile =
                precompile_by_tag(&Tag::JsonGetString).unwrap();
        }

        #[test]
        fn accepts_valid_precompile() {
            assert!(!is_time_dependent(&JSON_GET_STRING));
        }

        #[test]
        fn rejects_invalid_precompile() {
            assert!(is_time_dependent(&WEB_PROOF));
            assert!(is_time_dependent(&EMAIL_PROOF));
        }
    }
}
