use p256::PublicKey;
use pkcs8::DecodePublicKey;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize, Serializer};
use tlsn_core::proof::TlsProof;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WebProof {
    #[serde(
        serialize_with = "serialize_public_key",
        deserialize_with = "deserialize_public_key_from_pem_string"
    )]
    pub notary_pub_key: PublicKey,
    pub tls_proof: TlsProof,
}

fn deserialize_public_key_from_pem_string<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    let key_pem = String::deserialize(deserializer)?;
    PublicKey::from_public_key_pem(&key_pem).map_err(serde::de::Error::custom)
}

pub fn serialize_public_key<S>(key: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string = PublicKey::to_string(key);
    serializer.serialize_str(&string)
}

#[cfg(test)]
mod web_proof_serialization {
    use super::*;
    use crate::fixtures::{tls_proof_example, NOTARY_PUB_KEY_PEM_EXAMPLE};

    #[test]
    fn serde_json() {
        let web_proof = WebProof {
            notary_pub_key: PublicKey::from_public_key_pem(&NOTARY_PUB_KEY_PEM_EXAMPLE.to_string())
                .unwrap(),
            tls_proof: tls_proof_example(),
        };
        let serialized = serde_json::to_string(&web_proof).unwrap();
        let deserialized: WebProof = serde_json::from_str(&serialized).unwrap();
        assert_eq!(web_proof.notary_pub_key, deserialized.notary_pub_key);
    }
}
