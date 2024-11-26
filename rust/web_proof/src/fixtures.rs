use core::str;
use std::fs;

use p256::PublicKey;
use pkcs8::DecodePublicKey;

use crate::web_proof::{PresentationJson, WebProof, WebProofV7};

pub const NOTARY_PUB_KEY_PEM_EXAMPLE: &str = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";
pub const NOTARY_PUB_KEY_PEM_EXAMPLE2: &str = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEBv36FI4ZFszJa0DQFJ3wWCXvVLFr\ncRzMG5kaTeHGoSzDu6cFqx3uEWYpFGo6C0EOUgf+mEgbktLrXocv5yHzKg==\n-----END PUBLIC KEY-----\n";

pub fn read_fixture(path: &str) -> String {
    str::from_utf8(&fs::read(path).unwrap())
        .unwrap()
        .to_string()
        .replace('\n', "\r\n")
}

pub fn load_web_proof_fixture(proof_path: &str, notary_pub_key_pem: &str) -> WebProof {
    WebProof {
        presentation: serde_json::from_str(&read_fixture(proof_path)).unwrap(),
        notary_pub_key: PublicKey::from_public_key_pem(notary_pub_key_pem).unwrap(),
    }
}

pub fn load_web_proof_v7_fixture(proof_path: &str, notary_pub_key_pem: &str) -> WebProofV7 {
    let presentation_json = read_fixture(proof_path);
    let presentation_json: PresentationJson = serde_json::from_str(&presentation_json).unwrap();

    WebProofV7 {
        presentation_json,
        notary_pub_key: PublicKey::from_public_key_pem(notary_pub_key_pem).unwrap(),
    }
}
