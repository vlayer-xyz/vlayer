use rsa::{
    RsaPrivateKey,
    pkcs1v15::{self, SigningKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePublicKey},
    sha2::Sha256,
    signature::{Keypair, RandomizedSigner, SignatureEncoding},
};

use crate::{PublicKey, Signature, common::to_payload::ToPayload};

const PRIV_KEY: &str = include_str!("../../assets/private_key.pem");

#[derive(Clone)]
pub struct Signer {
    key: SigningKey<Sha256>,
}

impl Signer {
    #[allow(clippy::expect_used)]
    pub fn new(priv_key: &str) -> Self {
        let key = RsaPrivateKey::from_pkcs8_pem(priv_key).expect("Failed to decode private key");
        let key = SigningKey::<_>::new(key);

        Self { key }
    }
}

impl Default for Signer {
    fn default() -> Self {
        Self::new(PRIV_KEY)
    }
}

impl Signer {
    pub(crate) fn sign<P: ToPayload>(&self, payload: &P) -> Signature {
        let mut rng = rand::thread_rng();
        let signature = self.key.sign_with_rng(&mut rng, &payload.to_payload());
        Signature(signature.to_bytes().into())
    }

    #[allow(clippy::expect_used)]
    pub fn public_key(&self) -> PublicKey {
        let pub_key = self
            .key
            .verifying_key()
            .to_public_key_der()
            .expect("Failed to encode public key");
        PublicKey(pub_key.into_vec().into())
    }
}

impl From<PublicKey> for pkcs1v15::VerifyingKey<Sha256> {
    #[allow(clippy::expect_used)]
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
        let signer = Signer::default();
        let signature = signer.sign(&"alamakota");

        let pub_key = pub_key();
        let signature = rsa::pkcs1v15::Signature::try_from(signature.0.as_ref()).unwrap();
        let verification_result = pub_key.verify(br#""alamakota""#, &signature);

        assert!(verification_result.is_ok())
    }

    #[test]
    fn pub_key_can_verify_signature() {
        let signer = Signer::default();
        let signature = signer.sign(&"alamakota");
        let signature = rsa::pkcs1v15::Signature::try_from(signature.0.as_ref()).unwrap();

        let pub_key: VerifyingKey<Sha256> = signer.public_key().into();
        let verification_result = pub_key.verify(br#""alamakota""#, &signature);

        assert!(verification_result.is_ok())
    }
}
