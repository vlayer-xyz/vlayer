use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tlsn_core::{
    CryptoProvider,
    connection::ServerName,
    presentation::{Presentation, PresentationError, PresentationOutput},
    signing::VerifyingKey,
};

use crate::{request_transcript::RequestTranscript, response_transcript::ResponseTranscript};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct WebProof {
    #[serde(rename = "presentationJson")]
    pub presentation_json: PresentationJSON,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct PresentationJSON {
    pub version: String,
    pub data: String,
    pub meta: PresentationJsonMeta,
}

impl WebProof {
    pub(crate) fn verify(
        self,
    ) -> Result<(RequestTranscript, ResponseTranscript, ServerName, VerifyingKey), VerificationError>
    {
        let provider = CryptoProvider::default();

        let presentation = Presentation::try_from(self)?;
        let verifying_key = presentation.verifying_key().clone();

        let PresentationOutput {
            transcript,
            server_name,
            ..
        } = presentation.verify(&provider)?;

        let transcript = transcript.ok_or(VerificationError::EmptyTranscript)?;

        Ok((
            RequestTranscript::new(transcript.sent_unsafe().to_vec()),
            ResponseTranscript::new(transcript.received_unsafe().to_vec()),
            server_name.ok_or(VerificationError::NoServerName)?,
            verifying_key,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlTestMode {
    Full,
    Prefix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyRedactionMode {
    Disabled,
    EnabledUnsafe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub body_redaction_mode: BodyRedactionMode,
    pub url_test_mode: UrlTestMode,
}

impl Config {
    pub fn new(
        body_redaction_mode: impl Into<BodyRedactionMode>,
        url_test_mode: impl Into<UrlTestMode>,
    ) -> Self {
        Self {
            body_redaction_mode: body_redaction_mode.into(),
            url_test_mode: url_test_mode.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PresentationJsonMeta {
    #[serde(rename = "notaryUrl")]
    pub notary_url: Option<String>,
    #[serde(rename = "websocketProxyUrl")]
    pub websocket_proxy_url: Option<String>,
    #[serde(rename = "pluginUrl")]
    pub plugin_url: Option<String>,
}

impl TryFrom<WebProof> for Presentation {
    type Error = DeserializeError;

    fn try_from(web_proof: WebProof) -> Result<Self, DeserializeError> {
        let bytes = hex::decode(&web_proof.presentation_json.data)?;
        let presentation = bincode::deserialize(&bytes)?;
        Ok(presentation)
    }
}

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("Hex decode error: {0}")]
    HexDecode(#[from] hex::FromHexError),
    #[error("Bincode deserialize error: {0}")]
    Bincode(#[from] bincode::Error),
}

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("No server name found in the presentation")]
    NoServerName,

    #[error("Presentation error: {0}")]
    Presentation(#[from] PresentationError),

    #[error("Notary public key serialization error: {0}")]
    PublicKeySerialization(#[from] pkcs8::spki::Error),

    #[error("Empty transcript")]
    EmptyTranscript,

    #[error("Deserialization error: {0}")]
    Deserialize(#[from] DeserializeError),
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use k256::PublicKey;
    use pkcs8::DecodePublicKey;
    use tlsn_core::signing::KeyAlgId;

    use super::*;
    use crate::fixtures::{NOTARY_PUB_KEY_PEM_EXAMPLE, load_web_proof_fixture};

    const WEB_PROOF_BAD_SIGNATURE: &str =
        include_str!(".././testdata/web_proof_bad_signature.json");

    #[test]
    fn serialize_deserialize_web_proof() {
        let proof = load_web_proof_fixture();

        let serialized = serde_json::to_string(&proof).unwrap();
        let deserialized: WebProof = serde_json::from_str(&serialized).unwrap();

        assert_eq!(proof, deserialized);
    }

    #[test]
    fn fail_verification_session_error() {
        let invalid_proof: WebProof = serde_json::from_str(WEB_PROOF_BAD_SIGNATURE).unwrap();

        assert!(matches!(
            invalid_proof.verify(),
            Err(VerificationError::Presentation(err)) if err.to_string() == "presentation error: attestation error caused by: attestation proof error: signature error caused by: signature verification failed: invalid secp256k1 signature"
        ));
    }

    #[test]
    fn success_verification() {
        let proof = load_web_proof_fixture();
        let (request, response, _, _) = proof.verify().unwrap();

        assert_snapshot!("sent_request", String::from_utf8(request.transcript).unwrap());
        assert_snapshot!("received_response", String::from_utf8(response.transcript).unwrap());
    }

    #[test]
    fn success_get_server_name() {
        let proof = load_web_proof_fixture();
        let (_, _, server_name, _) = proof.verify().unwrap();
        assert_eq!(server_name.as_str(), "lotr-api.online");
    }

    #[test]
    fn success_get_notary_verifying_key() {
        let proof = load_web_proof_fixture();
        let (_, _, _, verifying_key) = proof.verify().unwrap();
        assert_eq!(
            PublicKey::from_sec1_bytes(verifying_key.data.as_ref()).unwrap(),
            PublicKey::from_public_key_pem(NOTARY_PUB_KEY_PEM_EXAMPLE).unwrap()
        );
    }

    #[test]
    fn deserialize_presentation() {
        let web_proof: WebProof = load_web_proof_fixture();

        let presentation: Presentation = Presentation::try_from(web_proof).unwrap();
        assert_eq!(presentation.verifying_key().alg, KeyAlgId::K256);
    }
}
