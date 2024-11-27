use p256::PublicKey;
use pkcs8::{DecodePublicKey, EncodePublicKey, LineEnding};
use serde::{de::Deserializer, Deserialize, Serialize, Serializer};
use thiserror::Error;
use tlsn_core::{
    connection::ServerName,
    presentation::{Presentation, PresentationError, PresentationOutput},
    CryptoProvider,
};

use crate::{request_transcript::RequestTranscript, response_transcript::ResponseTranscript};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebProof {
    #[serde(
        deserialize_with = "deserialize_public_key_from_pem_string",
        serialize_with = "serialize_public_key_to_pem_string"
    )]
    pub notary_pub_key: PublicKey,
    pub presentation: Presentation,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebProofV7 {
    #[serde(
        deserialize_with = "deserialize_public_key_from_pem_string",
        serialize_with = "serialize_public_key_to_pem_string"
    )]
    pub notary_pub_key: PublicKey,
    pub presentation_json: PresentationJson,
}

impl From<WebProofV7> for WebProof {
    fn from(web_proof_v7: WebProofV7) -> Self {
        let presentation = Presentation::from(web_proof_v7.presentation_json);
        Self {
            notary_pub_key: web_proof_v7.notary_pub_key,
            presentation,
        }
    }
}

impl WebProof {
    pub(crate) fn verify(
        self,
    ) -> Result<(RequestTranscript, ResponseTranscript, ServerName), VerificationError> {
        let provider = CryptoProvider::default();

        let PresentationOutput {
            transcript,
            server_name,
            ..
        } = self.presentation.verify(&provider)?;

        let transcript = transcript.unwrap();

        Ok((
            RequestTranscript::new(transcript.sent_unsafe().to_vec()),
            ResponseTranscript::new(transcript.received_unsafe().to_vec()),
            server_name.ok_or(VerificationError::NoServerName)?,
        ))
    }

    pub fn get_notary_pub_key(&self) -> Result<String, pkcs8::spki::Error> {
        self.notary_pub_key.to_public_key_pem(LineEnding::LF)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PresentationJson {
    pub version: String,
    pub data: String,
    pub meta: PresentationJsonMeta,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PresentationJsonMeta {
    #[serde(rename = "notaryUrl")]
    pub(crate) notary_url: Option<String>,
    #[serde(rename = "websocketProxyUrl")]
    pub(crate) websocket_proxy_url: Option<String>,
    #[serde(rename = "pluginUrl")]
    pub(crate) plugin_url: Option<String>,
}

impl From<PresentationJson> for Presentation {
    fn from(presentation_json: PresentationJson) -> Self {
        let bytes = hex::decode(presentation_json.data).unwrap();
        bincode::deserialize(&bytes).unwrap()
    }
}

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("No server name found in the presentation")]
    NoServerName,

    #[error("Presentation error: {0}")]
    Presentation(#[from] PresentationError),

    #[error("Notary public key serialization error: {0}")]
    PublicKeySerialization(#[from] pkcs8::spki::Error),
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
    use tlsn_core::signing::KeyAlgId;

    use super::*;
    use crate::fixtures::{load_web_proof_fixture, read_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

    #[test]
    fn serialize_deserialize_web_proof() {
        let proof = load_web_proof_fixture(
            "./testdata/swapi_presentation_0.1.0-alpha.7.json",
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        );

        let serialized = serde_json::to_string(&proof).unwrap();
        let deserialized: WebProof = serde_json::from_str(&serialized).unwrap();

        // TlsProofs don't derive Eq, so we compare only notary_pub_key from WebProof structure
        // Comparing notary_pub_key is more important because its (de)serialization is custom
        assert_eq!(proof.notary_pub_key, deserialized.notary_pub_key);
    }

    #[test]
    fn fail_verification_session_error() {
        let invalid_proof = load_web_proof_fixture(
            "./testdata/swapi_presentation_0.1.0-alpha.7.invalid_signature.json",
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        );
        assert!(matches!(
            invalid_proof.verify(),
            Err(VerificationError::Presentation(err)) if err.to_string() == "presentation error: attestation error caused by: attestation proof error: signature error caused by: signature verification failed: secp256k1 signature verification failed"
        ));
    }

    #[test]
    fn fail_verification_invalid_merkl_prof() {
        let invalid_proof = load_web_proof_fixture(
            "./testdata/swapi_presentation_0.1.0-alpha.7.invalid_merkle_proof.json",
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        );
        assert!(matches!(
            invalid_proof.verify(),
            Err(VerificationError::Presentation(err)) if err.to_string() == "presentation error: attestation error caused by: attestation proof error: body proof error caused by: merkle error: invalid merkle proof"
        ));
    }

    #[test]
    fn success_verification() {
        let proof = load_web_proof_fixture(
            "./testdata/swapi_presentation_0.1.0-alpha.7.json",
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        );
        let (request, response, _) = proof.verify().unwrap();

        assert_eq!(
            String::from_utf8(request.transcript).unwrap(),
            read_fixture("./testdata/swapi_request.txt")
        );
        assert_eq!(
            String::from_utf8(response.transcript).unwrap(),
            read_fixture("./testdata/swapi_response.txt")
        );
    }

    #[test]
    fn success_get_server_name() {
        let proof = load_web_proof_fixture(
            "./testdata/swapi_presentation_0.1.0-alpha.7.json",
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        );
        let (_, _, server_name) = proof.verify().unwrap();
        assert_eq!(server_name.as_str(), "swapi.dev");
    }

    #[test]
    fn success_get_notary_pub_key() {
        let proof = load_web_proof_fixture(
            "./testdata/swapi_presentation_0.1.0-alpha.7.json",
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        );
        assert_eq!(proof.get_notary_pub_key().unwrap(), NOTARY_PUB_KEY_PEM_EXAMPLE);
    }

    #[test]
    fn deserialize_presentation() {
        let presentation_json = read_fixture("./testdata/presentation.json");
        let presentation_json: PresentationJson = serde_json::from_str(&presentation_json).unwrap();

        let presentation: Presentation = presentation_json.into();
        assert_eq!(presentation.verifying_key().alg, KeyAlgId::K256);
    }
}
