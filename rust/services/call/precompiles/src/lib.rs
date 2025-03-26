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
    u64_to_address,
};
use url_pattern::test as url_pattern_test;
use web_proof::verify as web_proof;

pub fn precompiles(is_vlayer_test: bool) -> Vec<Precompile> {
    let mut list = vec![
        generate_precompile!(0x100, web_proof, 1000, 10, Tag::WebProof),
        generate_precompile!(0x101, email_proof, 1000, 10, Tag::EmailProof),
        generate_precompile!(0x102, json_get_string, 1000, 10, Tag::JsonGetString),
        generate_precompile!(0x103, json_get_int, 1000, 10, Tag::JsonGetInt),
        generate_precompile!(0x104, json_get_bool, 1000, 10, Tag::JsonGetBool),
        generate_precompile!(0x105, json_get_array_length, 1000, 10, Tag::JsonGetArrayLength),
        generate_precompile!(0x110, regex_is_match, 1000, 10, Tag::RegexIsMatch),
        generate_precompile!(0x111, regex_capture, 1000, 10, Tag::RegexCapture),
        generate_precompile!(0x120, url_pattern_test, 1000, 10, Tag::UrlPatternTest),
    ];

    if is_vlayer_test {
        list.push(generate_precompile!(0x130, system::is_vlayer_test, 1000, 10, Tag::IsVlayerTest));
    }

    list
}

pub fn precompile_by_address(address: &Address, is_vlayer_test: bool) -> Option<Precompile> {
    precompiles(is_vlayer_test)
        .into_iter()
        .find(|precomp| precomp.address() == address)
}
