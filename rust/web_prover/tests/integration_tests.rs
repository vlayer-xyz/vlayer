#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod integration_tests {
    use rangeset::RangeSet;
    use regex::Regex;
    use tlsn_core::{connection::ServerName, presentation::Presentation};
    use web_prover::{
        Method, NotarizeParamsBuilder, NotaryConfig, NotaryConfigBuilder, RedactionConfig,
        generate_web_proof, verify_presentation,
    };

    const MAX_SENT_DATA_TOO_LOW: usize = 100;
    const MAX_RECV_DATA_TOO_LOW: usize = 100;

    fn notary_config() -> NotaryConfig {
        NotaryConfigBuilder::default()
            .host("127.0.0.1")
            .port(7047)
            .build()
            .unwrap()
    }

    fn notary_params_builder() -> NotarizeParamsBuilder {
        let mut builder = NotarizeParamsBuilder::default();
        builder
            .notary_config(notary_config())
            .server_domain("lotr-api.online")
            .server_host("127.0.0.1")
            .server_port(3011_u16)
            .headers([("Authorization", "s3cret_t0ken")]);
        builder
    }

    #[tokio::test]
    async fn test_limits_too_low() {
        let web_proof_result = generate_web_proof(
            notary_params_builder()
                .uri("/auth_header_require")
                .body("body content")
                .max_sent_data(MAX_SENT_DATA_TOO_LOW)
                .max_recv_data(MAX_RECV_DATA_TOO_LOW)
                .build()
                .unwrap(),
        )
        .await;

        assert!(web_proof_result.is_err(), "Generate web proof should fail");
    }

    #[tokio::test]
    async fn test_full_roundtrip() {
        let web_proof_result = generate_web_proof(
            notary_params_builder()
                .uri("/auth_header_require")
                .body("body content")
                .build()
                .unwrap(),
        )
        .await;

        assert!(
            web_proof_result.is_ok(),
            "Generate web proof error: {:?}",
            web_proof_result.unwrap_err()
        );

        let verification_result =
            verify_presentation(to_presentation(&web_proof_result.unwrap())).unwrap();

        assert_eq!(
            verification_result.sent,
            "GET /auth_header_require HTTP/1.1\r\nauthorization: s3cret_t0ken\r\nhost: lotr-api.online\r\naccept: */*\r\naccept-encoding: identity\r\nconnection: close\r\nuser-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36\r\ncontent-length: 12\r\n\r\nbody content"
        );
        let regex = Regex::new(
            "^HTTP/1\\.1 200 OK\\r\\nAccess-Control-Allow-Credentials: true\\r\\nVary: \\*\\r\\nAccess-Control-Allow-Origin: \\*\\r\\nAccess-Control-Allow-Methods: GET\\r\\nAccess-Control-Allow-Headers: authorization, host, accept, accept-encoding, connection, user-agent, content-length\\r\\nAccess-Control-Expose-Headers: authorization, host, accept, accept-encoding, connection, user-agent, content-length\\r\\nContent-Type: application/json;charset=utf-8\\r\\nDate: [A-Za-z]{3}, \\d{2} [A-Za-z]{3} \\d{4} \\d{2}:\\d{2}:\\d{2} GMT\\r\\nContent-Length: \\d+\\r\\n\\r\\n\\{\"success\":true,\"name\":\"Tom Bombadil\",\"greeting\":\"Old Tom Bombadil is a merry fellow!\"}$",
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
        let web_proof_result = generate_web_proof(
            notary_params_builder()
                .uri("/auth_header_require?param1=value1&param2=value2")
                .redaction_config_fn(|_| RedactionConfig {
                    sent: RangeSet::from([0..10, 20..30, 100..200]),
                    recv: RangeSet::from([0..10, 20..30, 100..200]),
                })
                .build()
                .unwrap(),
        )
        .await;

        assert!(
            web_proof_result.is_ok(),
            "Generate web proof error: {:?}",
            web_proof_result.unwrap_err()
        );

        let verification_result =
            verify_presentation(to_presentation(&web_proof_result.unwrap())).unwrap();

        assert_eq!(
            verification_result.sent,
            "GET /auth_XXXXXXXXXXuire?paramXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXtr-api.online\r\naccept: */*\r\naccept-encoding: identity\r\nconnection: close\r\nuser-agent: Mozilla/5.0 (XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        );
        assert_eq!(
            verification_result.recv,
            "HTTP/1.1 2XXXXXXXXXXess-ControXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXcess-Control-Allow-Methods: GET\r\nAccess-Control-Allow-Headers: authorization, host, accept, accept-eXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        );
    }

    #[tokio::test]
    async fn test_post_with_no_redaction() {
        let body = serde_json::to_vec(&serde_json::json!({"name": "Saruman"})).unwrap();
        let params = notary_params_builder()
            .uri("/auth_header_require")
            .headers([("Authorization", "s3cret_t0ken"), ("content-type", "application/json")])
            .body(body)
            .method(Method::POST)
            .build()
            .unwrap();
        let web_proof_result = generate_web_proof(params).await;

        assert!(
            web_proof_result.is_ok(),
            "Generate web proof error: {:?}",
            web_proof_result.unwrap_err()
        );

        let verification_result =
            verify_presentation(to_presentation(&web_proof_result.unwrap())).unwrap();

        assert_eq!(
            verification_result.sent,
            "POST /auth_header_require HTTP/1.1\r\nauthorization: s3cret_t0ken\r\ncontent-type: application/json\r\nhost: lotr-api.online\r\naccept: */*\r\naccept-encoding: identity\r\nconnection: close\r\nuser-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36\r\ncontent-length: 18\r\n\r\n{\"name\":\"Saruman\"}"
        );
        let regex = Regex::new(
            "^HTTP/1\\.1 200 OK\\r\\nAccess-Control-Allow-Credentials: true\\r\\nVary: \\*\\r\\nAccess-Control-Allow-Origin: \\*\\r\\nAccess-Control-Allow-Methods: POST\\r\\nAccess-Control-Allow-Headers: authorization, content-type, host, accept, accept-encoding, connection, user-agent, content-length\\r\\nAccess-Control-Expose-Headers: authorization, content-type, host, accept, accept-encoding, connection, user-agent, content-length\\r\\nContent-Type: application/json;charset=utf-8\\r\\nDate: [A-Za-z]{3}, \\d{2} [A-Za-z]{3} \\d{4} \\d{2}:\\d{2}:\\d{2} GMT\\r\\nContent-Length: \\d+\\r\\n\\r\\n\\{\"success\":true,\"greeting\":\"Hello, Saruman!\"}$",
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

    fn to_presentation(web_proof_result: &str) -> Presentation {
        let parsed: serde_json::Value = serde_json::from_str(web_proof_result).unwrap();
        let hex_data = parsed["presentationJson"]["data"].as_str().unwrap();
        bincode::deserialize(&hex::decode(hex_data).unwrap()).unwrap()
    }
}
