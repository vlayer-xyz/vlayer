use rsa::{
    pkcs1v15::SigningKey,
    pkcs8::DecodePrivateKey,
    sha2::Sha256,
    signature::{RandomizedSigner, SignatureEncoding},
    RsaPrivateKey,
};
use serde::Serialize;
use serde_with::{base64::Base64, serde_as};

const PRIV_KEY: &str = include_str!("../../assets/private_key.pem");

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

    pub fn sign<P: ToSignablePayload>(&self, payload: &P) -> Signature {
        let mut rng = rand::thread_rng();
        let signature = self.key.sign_with_rng(&mut rng, &payload.to_payload());
        Signature(signature.to_bytes().into_vec())
    }
}

pub(super) trait ToSignablePayload {
    fn to_payload(&self) -> Vec<u8>;
}

impl<T: Serialize> ToSignablePayload for T {
    fn to_payload(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let formattter = olpc_cjson::CanonicalFormatter::new();
        let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formattter);
        self.serialize(&mut serializer)
            .expect("Failed to serialize signable struct");

        buf
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
        let verification_result = pub_key.verify(br#""alamakota""#, &signature);

        assert!(verification_result.is_ok())
    }
}
