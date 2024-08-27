use core::str;
use p256::PublicKey;
use pkcs8::DecodePublicKey;
use std::fs;
use tlsn_core::proof::TlsProof;

use crate::web_proof::WebProof;

pub const NOTARY_PUB_KEY_PEM_EXAMPLE: &str = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

pub fn read_fixture(path: &str) -> String {
    str::from_utf8(&fs::read(path).unwrap())
        .unwrap()
        .to_string()
        .replace('\n', "\r\n")
}

pub fn load_web_proof_fixture(proof_path: &str, notary_pub_key_pem: &str) -> WebProof {
    WebProof {
        tls_proof: serde_json::from_str(&read_fixture(proof_path)).unwrap(),
        notary_pub_key: PublicKey::from_public_key_pem(notary_pub_key_pem).unwrap(),
    }
}

pub fn tls_proof_example() -> TlsProof {
    serde_json::from_str(str::from_utf8(include_bytes!("../testdata/tls_proof.json")).unwrap())
        .unwrap()
}
