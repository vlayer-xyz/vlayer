use core::str;
use std::fs;

use crate::web_proof::WebProof;

#[cfg(test)]
pub(crate) mod tlsn_core_types;

pub const NOTARY_PUB_KEY_PEM_EXAMPLE: &str = "-----BEGIN PUBLIC KEY-----\nMDYwEAYHKoZIzj0CAQYFK4EEAAoDIgADZT9nJiwhGESLjwQNnZ2MsZ1xwjGzvmhF\nxFi8Vjzanlg=\n-----END PUBLIC KEY-----";
const WEB_PROOF_FIXTURE: &str = include_str!(".././testdata/web_proof.json");

pub fn read_fixture(path: &str) -> String {
    str::from_utf8(&fs::read(path).unwrap())
        .unwrap()
        .to_string()
        .replace('\n', "\r\n")
}

pub fn load_web_proof_fixture() -> WebProof {
    serde_json::from_str(WEB_PROOF_FIXTURE).unwrap()
}

#[cfg(test)]
pub(crate) mod utils {
    use tlsn_core::{
        attestation::Field,
        connection::ServerName,
        signing::{Signature, VerifyingKey},
    };

    use super::{
        WEB_PROOF_FIXTURE,
        tlsn_core_types::{AttestationProof, Body, BodyProof, Presentation, ServerIdentityProof},
    };
    use crate::web_proof::{PresentationJSON, PresentationJsonMeta, WebProof};

    pub(crate) fn load_web_proof_fixture_and_modify<F>(modify: F) -> WebProof
    where
        F: FnOnce(&Presentation) -> Presentation,
    {
        let web_proof: WebProof = serde_json::from_str(WEB_PROOF_FIXTURE).unwrap();
        let test_presentation: Presentation =
            bincode::deserialize(&hex::decode(web_proof.presentation_json.data).unwrap()).unwrap();

        let modified_presentation = modify(&test_presentation);

        let data = hex::encode(bincode::serialize(&modified_presentation).unwrap());

        WebProof {
            presentation_json: PresentationJSON {
                version: "0.1.0-alpha.7".to_string(),
                data,
                meta: PresentationJsonMeta {
                    notary_url: Some("wss://notary.pse.dev/v0.1.0-alpha.7/notarize?sessionId=47a8a400-a25f-4571-9825-714b6e4a6689".to_string()),
                    websocket_proxy_url: Some("ws://localhost:55688".to_string()),
                    plugin_url: None
                }
            }
        }
    }

    pub(crate) fn corrupt_signature(test_presentation: &Presentation) -> Presentation {
        Presentation {
            attestation: AttestationProof {
                signature: Signature {
                    alg: test_presentation.attestation.signature.alg,
                    data: vec![0; test_presentation.attestation.signature.data.len()],
                },
                ..test_presentation.attestation.clone()
            },
            ..test_presentation.clone()
        }
    }

    pub(crate) fn change_server_name(test_presentation: &Presentation) -> Presentation {
        Presentation {
            identity: Some(ServerIdentityProof {
                name: ServerName::new("api.y.com".to_string()),
                ..test_presentation.identity.clone().unwrap()
            }),
            ..test_presentation.clone()
        }
    }

    pub(crate) fn corrupt_verifying_key(test_presentation: &Presentation) -> Presentation {
        Presentation {
            attestation: AttestationProof {
                body: BodyProof {
                    body: Body {
                        verifying_key: Field {
                            data: VerifyingKey {
                                alg: test_presentation
                                    .attestation
                                    .body
                                    .body
                                    .verifying_key
                                    .data
                                    .alg,
                                data: vec![0; 32],
                            },
                            ..test_presentation
                                .attestation
                                .body
                                .body
                                .verifying_key
                                .clone()
                        },
                        ..test_presentation.attestation.body.body.clone()
                    },
                    ..test_presentation.attestation.body.clone()
                },
                ..test_presentation.attestation.clone()
            },
            ..test_presentation.clone()
        }
    }
}
