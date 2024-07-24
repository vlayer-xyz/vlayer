use serde::{Deserialize, Serialize};
use tlsn_core::proof::TlsProof;
use serde::de::Deserializer;
use p256::PublicKey;
use pkcs8::DecodePublicKey;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebProof {
    #[serde(deserialize_with = "deserialize_public_key_from_pem_string")]
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
