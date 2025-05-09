#![allow(clippy::unwrap_used)]

use core::str;
use std::fs;

use crate::web_proof::WebProof;

#[cfg(test)]
pub(crate) mod tlsn_core_types;

pub const NOTARY_PUB_KEY_PEM_EXAMPLE: &str = "-----BEGIN PUBLIC KEY-----\nMDYwEAYHKoZIzj0CAQYFK4EEAAoDIgADe0jxnBObaIj7Xjg6TXLCM1GG/VhY5650\nOrS/jgcbBuc=\n-----END PUBLIC KEY-----";
const WEB_PROOF_FIXTURE: &str = include_str!(".././testdata/web_proof.json");

pub fn read_fixture(path: &str) -> String {
    str::from_utf8(&fs::read(path).unwrap())
        .unwrap()
        .to_string()
        .replace('\n', "\r\n")
}

pub fn load_web_proof_fixture() -> WebProof {
    serde_json::from_str(WEB_PROOF_FIXTURE).unwrap()
}
