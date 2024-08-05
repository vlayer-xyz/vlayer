use core::str;
use p256::{ecdsa, PublicKey};
use pkcs8::DecodePublicKey;
use std::fs;
use tlsn_core::proof::TlsProof;

use crate::types::WebProof;

pub const NOTARY_PUB_KEY_PEM_EXAMPLE: &str = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

pub fn notary_pub_key_example() -> PublicKey {
    PublicKey::from_public_key_pem(NOTARY_PUB_KEY_PEM_EXAMPLE).unwrap()
}

pub fn tls_proof_example() -> TlsProof {
    serde_json::from_str(str::from_utf8(include_bytes!("../testdata/tls_proof.json")).unwrap())
        .unwrap()
}

pub fn load_web_proof_fixture(proof_path: &str, notary_pub_key_pem: &str) -> WebProof {
    WebProof {
        tls_proof: serde_json::from_str(&read_fixture(proof_path)).unwrap(),
        notary_pub_key: PublicKey::from_public_key_pem(notary_pub_key_pem).unwrap(),
    }
}

pub fn invalid_tls_proof_example() -> TlsProof {
    let mut tls_proof = tls_proof_example();

    let valid_signature = tls_proof.session.signature.unwrap();
    let invalid_signature = change_signature(valid_signature.clone());

    tls_proof.session.signature = Some(invalid_signature);
    tls_proof
}

pub fn read_fixture(path: &str) -> String {
    str::from_utf8(&fs::read(path).unwrap())
        .unwrap()
        .to_string()
        .replace('\n', "\r\n")
}

fn change_signature(mut signature: tlsn_core::Signature) -> tlsn_core::Signature {
    match signature {
        tlsn_core::Signature::P256(ref mut sig) => {
            let mut bytes = sig.to_bytes();
            bytes[0] = bytes[0].wrapping_add(1);
            let invalid_sig = ecdsa::Signature::from_bytes(&bytes).unwrap();
            tlsn_core::Signature::from(invalid_sig)
        }
        _ => signature,
    }
}
