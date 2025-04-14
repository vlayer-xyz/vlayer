#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod integration_tests {
    use std::{collections::HashMap, error::Error};

    use regex::Regex;
    use tlsn_core::{connection::ServerName, presentation::Presentation};
    use utils::range::RangeSet;
    use web_prover::{
        NotarizeParamsBuilder, NotaryConfig, RedactionConfig, generate_web_proof,
        verify_presentation,
    };

    #[tokio::test]
    async fn test_full_roundtrip() {
        let web_proof_result = Box::pin(generate_web_proof(
            NotarizeParamsBuilder::default()
                .notary_config(NotaryConfig::new("127.0.0.1".into(), 7047, "".into(), false))
                .server_domain("lotr-api.online")
                .server_host("127.0.0.1")
                .server_port(3011_u16)
                .uri("/auth_header_require")
                .headers(HashMap::from([("Authorization".to_string(), "s3cret_t0ken".to_string())]))
                .body("body content")
                .build()
                .unwrap(),
        ))
        .await;

        assert!(
            web_proof_result.is_ok(),
            "Generate web proof error: {:?}",
            web_proof_result.unwrap_err()
        );

        let verification_result = verify_presentation(to_presentation(&web_proof_result)).unwrap();

        assert_eq!(
            verification_result.sent,
            "GET /auth_header_require HTTP/1.1\r\nhost: lotr-api.online\r\naccept: */*\r\naccept-encoding: identity\r\nconnection: close\r\nuser-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36\r\nauthorization: s3cret_t0ken\r\ncontent-length: 12\r\n\r\nbody content"
        );
        let regex = Regex::new(
            "^HTTP/1\\.1 200 OK\\r\\nAccess-Control-Allow-Credentials: true\\r\\nVary: \\*\\r\\nAccess-Control-Allow-Origin: \\*\\r\\nAccess-Control-Allow-Methods: GET\\r\\nAccess-Control-Allow-Headers: host, accept, accept-encoding, connection, user-agent, authorization, content-length\\r\\nAccess-Control-Expose-Headers: host, accept, accept-encoding, connection, user-agent, authorization, content-length\\r\\nContent-Type: application/json;charset=utf-8\\r\\nDate: [A-Za-z]{3}, \\d{2} [A-Za-z]{3} \\d{4} \\d{2}:\\d{2}:\\d{2} GMT\\r\\nContent-Length: \\d+\\r\\n\\r\\n\\{\"success\":true,\"name\":\"Tom Bombadil\",\"greeting\":\"Old Tom Bombadil is a merry fellow!\"}$",
        );
        assert!(
            regex.unwrap().is_match(&verification_result.recv,),
            "Got response: {}",
            verification_result.recv,
        );
        assert_eq!(verification_result.server_name, ServerName::new("lotr-api.online".to_string()));
        assert_eq!(
            verification_result.key,
            "037b48f19c139b6888fb5e383a4d72c2335186fd5858e7ae743ab4bf8e071b06e7"
        );
    }

    #[tokio::test]
    async fn test_redaction() {
        let web_proof_result = Box::pin(generate_web_proof(
            NotarizeParamsBuilder::default()
                .notary_config(NotaryConfig::new("127.0.0.1".into(), 7047, "".into(), false))
                .server_domain("lotr-api.online")
                .server_host("127.0.0.1")
                .server_port(3011_u16)
                .uri("/auth_header_require?param1=value1&param2=value2")
                .headers(HashMap::from([("Authorization".to_string(), "s3cret_t0ken".to_string())]))
                .redaction_config_fn(|_| RedactionConfig {
                    sent: RangeSet::from([0..10, 20..30, 100..200]),
                    recv: RangeSet::from([0..10, 20..30, 100..200]),
                })
                .build()
                .unwrap(),
        ))
        .await;

        assert!(
            web_proof_result.is_ok(),
            "Generate web proof error: {:?}",
            web_proof_result.unwrap_err()
        );

        let verification_result = verify_presentation(to_presentation(&web_proof_result)).unwrap();

        assert_eq!(
            verification_result.sent,
            "GET /auth_XXXXXXXXXXuire?paramXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXccept-encoding: identity\r\nconnection: close\r\nuser-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKitXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        );
        assert_eq!(
            verification_result.recv,
            "HTTP/1.1 2XXXXXXXXXXess-ControXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXcess-Control-Allow-Methods: GET\r\nAccess-Control-Allow-Headers: host, accept, accept-encoding, connecXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        );
    }

    fn to_presentation(web_proof_result: &Result<String, Box<dyn Error>>) -> Presentation {
        let json_str = web_proof_result.as_ref().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let hex_data = parsed["presentationJson"]["data"].as_str().unwrap();
        bincode::deserialize(&hex::decode(hex_data).unwrap()).unwrap()
    }
}
