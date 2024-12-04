use core::str;
use std::fs;

use p256::PublicKey;
use pkcs8::DecodePublicKey;
use tlsn_core::{
    attestation::Field,
    connection::ServerName,
    signing::{Signature, VerifyingKey},
};
use tlsn_core_types::{
    BodyProof, ServerIdentityProof, TestAttestationProof, TestBody, TestPresentation,
};

use crate::web_proof::{PresentationJson, PresentationJsonMeta, WebProof};

pub mod tlsn_core_types;

pub const NOTARY_PUB_KEY_PEM_EXAMPLE: &str = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";
const PRESENTATION_FIXTURE: &str = include_str!(".././testdata/presentation.json");

pub fn read_fixture(path: &str) -> String {
    str::from_utf8(&fs::read(path).unwrap())
        .unwrap()
        .to_string()
        .replace('\n', "\r\n")
}

pub fn load_web_proof_fixture() -> WebProof {
    load_web_proof_fixture_and_modify(std::clone::Clone::clone)
}

pub fn load_web_proof_fixture_and_modify<F>(modify: F) -> WebProof
where
    F: FnOnce(&TestPresentation) -> TestPresentation,
{
    let presentation_json: PresentationJson = serde_json::from_str(PRESENTATION_FIXTURE).unwrap();
    let test_presentation: TestPresentation =
        bincode::deserialize(&hex::decode(presentation_json.data).unwrap()).unwrap();

    let modified_presentation = modify(&test_presentation);

    let data = hex::encode(bincode::serialize(&modified_presentation).unwrap());

    WebProof {
        presentation_json: PresentationJson {
            version: "0.1.0-alpha.7".to_string(),
            data,
            meta: PresentationJsonMeta {
              notary_url: Some("wss://notary.pse.dev/v0.1.0-alpha.7/notarize?sessionId=47a8a400-a25f-4571-9825-714b6e4a6689".to_string()),
              websocket_proxy_url: Some("ws://localhost:55688".to_string()),
              plugin_url: None
          }
        },
        notary_pub_key: PublicKey::from_public_key_pem(NOTARY_PUB_KEY_PEM_EXAMPLE).unwrap(),
    }
}

pub fn corrupt_signature(test_presentation: &TestPresentation) -> TestPresentation {
    TestPresentation {
        attestation: TestAttestationProof {
            signature: Signature {
                alg: test_presentation.attestation.signature.alg,
                data: vec![0; test_presentation.attestation.signature.data.len()],
            },
            ..test_presentation.attestation.clone()
        },
        ..test_presentation.clone()
    }
}

pub fn change_server_name(test_presentation: &TestPresentation) -> TestPresentation {
    TestPresentation {
        identity: Some(ServerIdentityProof {
            name: ServerName::new("api.y.com".to_string()),
            ..test_presentation.identity.clone().unwrap()
        }),
        ..test_presentation.clone()
    }
}

pub fn corrupt_verifying_key(test_presentation: &TestPresentation) -> TestPresentation {
    TestPresentation {
        attestation: TestAttestationProof {
            body: BodyProof {
                body: TestBody {
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
