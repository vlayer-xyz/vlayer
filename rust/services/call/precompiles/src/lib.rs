pub mod email_proof;
mod helpers;
mod json;
pub mod precompile;
mod regex;
pub mod system;
pub mod url_pattern;
mod web_proof;

use alloy_primitives::{Address, Bytes};
use email_proof::verify as email_proof;
use helpers::generate_precompile;
use json::{
    get_array_length as json_get_array_length, get_bool as json_get_bool, get_int as json_get_int,
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
        generate_precompile!("web_proof", web_proof, 1000, 10, Tag::WebProof),
        generate_precompile!("email_proof", email_proof, 1000, 10, Tag::EmailProof),
        generate_precompile!("json_get_string", json_get_string, 1000, 10, Tag::JsonGetString),
        generate_precompile!("json_get_int", json_get_int, 1000, 10, Tag::JsonGetInt),
        generate_precompile!("json_get_bool", json_get_bool, 1000, 10, Tag::JsonGetBool),
        generate_precompile!(
            "json_get_array_length",
            json_get_array_length,
            1000,
            10,
            Tag::JsonGetArrayLength
        ),
        generate_precompile!("regex_is_match", regex_is_match, 1000, 10, Tag::RegexIsMatch),
        generate_precompile!("regex_capture", regex_capture, 1000, 10, Tag::RegexCapture),
        generate_precompile!("url_pattern_test", url_pattern_test, 1000, 10, Tag::UrlPatternTest),
    ];

    if is_vlayer_test {
        list.push(generate_precompile!(
            "is_vlayer_test",
            system::is_vlayer_test,
            1000,
            10,
            Tag::IsVlayerTest
        ));
    }

    list
}

pub fn precompile_by_address(address: &Address, is_vlayer_test: bool) -> Option<Precompile> {
    precompiles(is_vlayer_test)
        .into_iter()
        .find(|precomp| precomp.address() == address)
}

pub fn precompile_by_name(name: &str) -> Option<Precompile> {
    let name_snake = name.trim().to_ascii_lowercase();
    precompiles(false)
        .into_iter()
        .find(|precomp| precomp.tag().to_string().to_ascii_lowercase() == name_snake)
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
    use alloy_primitives::hex;

    use super::*;

    mod verify_precompile_allowed_in_travel_call {
        use lazy_static::lazy_static;
        use revm::precompile::u64_to_address;

        use super::*;

        lazy_static! {
            static ref WEB_PROOF: Precompile =
                precompile_by_address(&u64_to_address(0x100), false).unwrap();
            static ref EMAIL_PROOF: Precompile =
                precompile_by_address(&u64_to_address(0x101), false).unwrap();
            static ref JSON_GET_STRING: Precompile =
                precompile_by_address(&u64_to_address(0x102), false).unwrap();
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

    #[test]
    fn log_all_precompile_addresses() {
        let precompiles = super::precompiles(true); // include `is_vlayer_test`

        println!("\nPrecompile Addresses:");
        for precompile in precompiles {
            let address = precompile.address();
            let tag = format!("{:?}", precompile.tag());
            println!("  {:<25} => 0x{}", tag, hex::encode(address.as_slice()));
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn finds_existing_precompile_by_name() {
            let precompile = precompile_by_name("web_proof");
            assert!(precompile.is_some(), "Expected 'web_proof' precompile to exist");

            let tag = precompile.unwrap().tag();
            assert_eq!(tag, Tag::WebProof);
        }

        #[test]
        fn returns_none_for_nonexistent_precompile() {
            let precompile = precompile_by_name("not_a_real_precompile");
            assert!(precompile.is_none(), "Expected None for an invalid precompile name");
        }

        #[test]
        fn matches_case_sensitive_name() {
            let correct_case = precompile_by_name("web_proof");
            let wrong_case = precompile_by_name("Web_Proof");

            assert!(correct_case.is_some(), "Expected correct casing to match");
            assert!(wrong_case.is_none(), "Expected wrong casing to return None");
        }

        #[test]
        fn all_tags_are_accessible_by_name() {
            for precompile in precompiles(false) {
                let name = precompile.tag().to_string();
                let found = precompile_by_name(&name);
                assert!(found.is_some(), "Expected precompile '{}' to be found by name", name);
                assert_eq!(
                    found.unwrap().tag(),
                    precompile.tag(),
                    "Mismatched tags for '{}'",
                    name
                );
            }
        }
    }
}
