#[macro_use]
mod helpers;
pub mod email_proof;
mod json;
mod regex;
pub mod url_pattern;
mod web_proof;

use email_proof::verify as email_proof;
use json::{
    get_array_length as json_get_array_length, get_bool as json_get_bool, get_int as json_get_int,
    get_string as json_get_string,
};
use regex::{capture as regex_capture, is_match as regex_is_match};
use revm::precompile::PrecompileWithAddress;
use url_pattern::test as url_pattern_test;
use web_proof::verify as web_proof;

const NUM_PRECOMPILES: usize = 9;

#[rustfmt::skip]
pub const PRECOMPILES: [PrecompileWithAddress; NUM_PRECOMPILES] = generate_precompiles![
    (0x100, web_proof,             1000, 10),
    (0x101, email_proof,           1000, 10),
    (0x102, json_get_string,       1000, 10),
    (0x103, json_get_int,          1000, 10),
    (0x104, json_get_bool,         1000, 10),
    (0x105, json_get_array_length, 1000, 10),
    (0x110, regex_is_match,        1000, 10),
    (0x111, regex_capture,         1000, 10),
    (0x120, url_pattern_test,      1000, 10),
];
