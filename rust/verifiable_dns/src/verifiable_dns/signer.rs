use bytes::Bytes;
use rsa::{
    pkcs1v15::{self, SigningKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePublicKey},
    sha2::Sha256,
    signature::{Keypair, RandomizedSigner, SignatureEncoding},
    RsaPrivateKey,
};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};

const PRIV_KEY: &str = include_str!("../../assets/private_key.pem");

#[serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Signature(#[serde_as(as = "Base64")] pub(crate) Bytes);

#[serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PublicKey(#[serde_as(as = "Base64")] pub(crate) Bytes);

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
        Signature(signature.to_bytes().into())
    }

    pub fn public_key(&self) -> PublicKey {
        let pub_key = self
            .key
            .verifying_key()
            .to_public_key_der()
            .expect("Failed to encode public key");
        PublicKey(pub_key.into_vec().into())
    }
}

pub(crate) trait ToSignablePayload {
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

impl From<PublicKey> for pkcs1v15::VerifyingKey<Sha256> {
    fn from(value: PublicKey) -> Self {
        Self::from_public_key_der(&value.0).expect("Failed to decode public key")
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
        let signature = rsa::pkcs1v15::Signature::try_from(signature.0.as_ref()).unwrap();
        let verification_result = pub_key.verify(br#""alamakota""#, &signature);

        assert!(verification_result.is_ok())
    }

    #[test]
    fn pub_key_can_verify_signature() {
        let signer = Signer::new();
        let signature = signer.sign(&"alamakota");
        let signature = rsa::pkcs1v15::Signature::try_from(signature.0.as_ref()).unwrap();

        let pub_key: VerifyingKey<Sha256> = signer.public_key().into();
        let verification_result = pub_key.verify(br#""alamakota""#, &signature);

        assert!(verification_result.is_ok())
    }
}
