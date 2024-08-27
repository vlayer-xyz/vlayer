use p256::PublicKey;
use pkcs8::{DecodePublicKey, EncodePublicKey, LineEnding};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize, Serializer};
use thiserror::Error;
use tlsn_core::proof::{SessionProofError, SubstringsProofError, TlsProof};
use tlsn_core::ServerName;

use crate::request_transcript::RequestTranscript;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebProof {
    #[serde(
        deserialize_with = "deserialize_public_key_from_pem_string",
        serialize_with = "serialize_public_key_to_pem_string"
    )]
    pub notary_pub_key: PublicKey,
    pub tls_proof: TlsProof,
}

impl WebProof {
    pub(crate) fn verify(self) -> Result<RequestTranscript, VerificationError> {
        let TlsProof {
            session,
            substrings,
        } = self.tls_proof;

        session.verify_with_default_cert_verifier(self.notary_pub_key)?;
        let (sent, _) = substrings.verify(&session.header)?;

        Ok(RequestTranscript::new(sent))
    }

    pub fn get_server_name(&self) -> String {
        let ServerName::Dns(server_name) = &self.tls_proof.session.session_info.server_name;
        server_name.to_string()
    }
}

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Session proof error: {0}")]
    SessionProof(#[from] SessionProofError),

    #[error("Substrings proof error: {0}")]
    SubstringsProof(#[from] SubstringsProofError),
}

fn deserialize_public_key_from_pem_string<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    let key_pem = String::deserialize(deserializer)?;
    PublicKey::from_public_key_pem(&key_pem).map_err(serde::de::Error::custom)
}

fn serialize_public_key_to_pem_string<S>(key: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let key_pem = key
        .to_public_key_pem(LineEnding::LF)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&key_pem)
}

#[cfg(test)]
mod tests {
    use crate::fixtures::{load_web_proof_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

    #[test]
    fn fail_verification() {
        let invalid_proof = load_web_proof_fixture(
            "./testdata/invalid_tls_proof.json",
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        );
        assert!(invalid_proof.verify().is_err());
    }

    #[test]
    fn success_verification() {
        let proof = load_web_proof_fixture("./testdata/tls_proof.json", NOTARY_PUB_KEY_PEM_EXAMPLE);
        assert!(proof.verify().is_ok());
    }
}
