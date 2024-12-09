use k256::PublicKey;
use pkcs8::{EncodePublicKey, LineEnding};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tlsn_core::{
    connection::ServerName,
    presentation::{Presentation, PresentationError, PresentationOutput},
    signing::VerifyingKey,
    CryptoProvider,
};

use crate::{request_transcript::RequestTranscript, response_transcript::ResponseTranscript};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebProof {
    pub presentation_json: PresentationJson,
}

impl WebProof {
    pub(crate) fn verify(
        self,
    ) -> Result<(RequestTranscript, ResponseTranscript, ServerName), VerificationError> {
        let provider = CryptoProvider::default();

        let presentation = Presentation::from(self.presentation_json);

        let PresentationOutput {
            transcript,
            server_name,
            ..
        } = presentation.verify(&provider)?;

        let transcript = transcript.unwrap();

        Ok((
            RequestTranscript::new(transcript.sent_unsafe().to_vec()),
            ResponseTranscript::new(transcript.received_unsafe().to_vec()),
            server_name.ok_or(VerificationError::NoServerName)?,
        ))
    }

    pub fn get_notary_pub_key(&self) -> Result<String, pkcs8::spki::Error> {
        PublicKey::from_sec1_bytes(self.get_notary_verifying_key().data.as_ref())
            .unwrap()
            .to_public_key_pem(LineEnding::LF)
    }

    pub fn get_notary_verifying_key(&self) -> VerifyingKey {
        Presentation::from(self.presentation_json.clone())
            .verifying_key()
            .clone()
    }

    pub fn get_notary_verifying_key(&self) -> VerifyingKey {
        Presentation::from(self.presentation_json.clone())
            .verifying_key()
            .clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PresentationJson {
    pub(crate) version: String,
    pub(crate) data: String,
    pub(crate) meta: PresentationJsonMeta,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PresentationJsonMeta {
    #[serde(rename = "notaryUrl")]
    pub notary_url: Option<String>,
    #[serde(rename = "websocketProxyUrl")]
    pub websocket_proxy_url: Option<String>,
    #[serde(rename = "pluginUrl")]
    pub plugin_url: Option<String>,
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

#[cfg(test)]
mod tests {
    use pkcs8::DecodePublicKey;
    use tlsn_core::signing::KeyAlgId;

    use super::*;
    use crate::fixtures::{
        load_web_proof_fixture, read_fixture,
        utils::{corrupt_signature, corrupt_verifying_key, load_web_proof_fixture_and_modify},
        NOTARY_PUB_KEY_PEM_EXAMPLE,
    };

    #[test]
    fn serialize_deserialize_web_proof() {
        let proof = load_web_proof_fixture();

        let serialized = serde_json::to_string(&proof).unwrap();
        let deserialized: WebProof = serde_json::from_str(&serialized).unwrap();

        // TlsProofs don't derive Eq, so we compare only notary_pub_key from WebProof structure
        // Comparing notary_pub_key is more important because its (de)serialization is custom
        assert_eq!(proof.get_notary_pub_key(), deserialized.get_notary_pub_key());
    }

    #[test]
    fn fail_verification_session_error() {
        let invalid_proof = load_web_proof_fixture_and_modify(corrupt_signature);

        assert!(matches!(
            invalid_proof.verify(),
            Err(VerificationError::Presentation(err)) if err.to_string() == "presentation error: attestation error caused by: attestation proof error: signature error caused by: signature verification failed: invalid secp256k1 signature"
        ));
    }

    #[test]
    fn fail_verification_invalid_merkle_proof() {
        let invalid_proof = load_web_proof_fixture_and_modify(corrupt_verifying_key);
        assert!(matches!(
            invalid_proof.verify(),
            Err(VerificationError::Presentation(err)) if err.to_string() == "presentation error: attestation error caused by: attestation proof error: body proof error caused by: merkle error: invalid merkle proof"
        ));
    }

    #[test]
    fn success_verification() {
        let proof = load_web_proof_fixture();
        let (request, response, _) = proof.verify().unwrap();

        assert_eq!(
            String::from_utf8(request.transcript).unwrap(),
            read_fixture("./testdata/sent_request.txt")
        );
        assert_eq!(
            String::from_utf8(response.transcript).unwrap(),
            read_fixture("./testdata/received_response.txt")
        );
    }

    #[test]
    fn success_get_server_name() {
        let proof = load_web_proof_fixture();
        let (_, _, server_name) = proof.verify().unwrap();
        assert_eq!(server_name.as_str(), "api.x.com");
    }

    #[test]
    fn success_get_notary_pub_key() {
        let proof = load_web_proof_fixture();
        assert_eq!(
            PublicKey::from_public_key_pem(&proof.get_notary_pub_key().unwrap()),
            PublicKey::from_public_key_pem(NOTARY_PUB_KEY_PEM_EXAMPLE)
        );
    }

    #[test]
    fn success_get_notary_verifying_key() {
        let proof = load_web_proof_fixture();

        let verifying_key = proof.get_notary_verifying_key();
        let public_key = PublicKey::from_public_key_pem(NOTARY_PUB_KEY_PEM_EXAMPLE).unwrap();
        let notary_public_key_sec1_bytes = public_key.to_sec1_bytes();

        assert_eq!(verifying_key.data, notary_public_key_sec1_bytes.as_ref());
        assert_eq!(verifying_key.alg, KeyAlgId::K256);
    }

    #[test]
    fn deserialize_presentation() {
        let presentation_json = read_fixture("./testdata/presentation.json");
        let presentation_json: PresentationJson = serde_json::from_str(&presentation_json).unwrap();

        let presentation: Presentation = presentation_json.into();
        assert_eq!(presentation.verifying_key().alg, KeyAlgId::K256);
    }
}
