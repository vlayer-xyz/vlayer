use rsa::{
    pkcs1v15::SigningKey,
    pkcs8::DecodePrivateKey,
    sha2::Sha256,
    signature::{RandomizedSigner, SignatureEncoding},
    RsaPrivateKey,
};
use serde::Serialize;
use serde_with::serde_as;

const PRIV_KEY: &str = include_str!("../../assets/private_key.pem");

pub(super) trait IntoSigningPayload {
    fn into_payload(self, now: u64) -> Vec<u8>;
}

use serde_with::base64::Base64;
#[serde_as]
#[derive(Serialize, Debug, PartialEq)]
pub struct Signature(#[serde_as(as = "Base64")] pub Vec<u8>);

#[derive(Clone)]
pub(super) struct Signer {
    key: SigningKey<Sha256>,
}

impl Signer {
    pub fn new() -> Self {
        let key = RsaPrivateKey::from_pkcs8_pem(PRIV_KEY).expect("Failed to decode private key");
        let key = SigningKey::<_>::new(key);

        Self { key }
    }

    pub fn sign<T: IntoSigningPayload + Clone>(&self, payload: &T) -> Signature {
        let mut rng = rand::thread_rng();
        let payload = payload.clone().into_payload(0);
        let signature = self.key.sign_with_rng(&mut rng, &payload);
        Signature(signature.to_bytes().into_vec())
    }
}

// TODO: move under cfg(test)
impl IntoSigningPayload for &str {
    fn into_payload(self, now: u64) -> Vec<u8> {
        format!("{now},{self}").into_bytes()
    }
}

#[cfg(test)]
mod tests {
    const PUB_KEY: &str = include_str!("../../assets/public_key.pem");

    use rsa::{pkcs1v15::VerifyingKey, pkcs8::DecodePublicKey, signature::Verifier};

    use super::*;

    fn pub_key() -> VerifyingKey<Sha256> {
        VerifyingKey::from_public_key_pem(PUB_KEY).unwrap()
    }

    #[test]
    fn can_sign() {
        let signer = Signer::new();
        let signature = signer.sign(&"alamakota");

        let pub_key = pub_key();
        let signature = rsa::pkcs1v15::Signature::try_from(signature.0.as_slice()).unwrap();
        let verification_result = pub_key.verify(b"0,alamakota", &signature);

        assert!(verification_result.is_ok())
    }
}
