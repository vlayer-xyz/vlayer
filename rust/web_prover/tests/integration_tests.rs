#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod integration_tests {
    use std::error::Error;

    use regex::Regex;
    use tlsn_core::{connection::ServerName, presentation::Presentation};
    use web_prover::{generate_web_proof, verify_presentation};

    #[tokio::test]
    async fn test_full_roundtrip() {
        let web_proof_result = Box::pin(generate_web_proof(
            "127.0.0.1",
            7047,
            "lotr-api.online",
            "127.0.0.1",
            3011,
            "/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
        ))
        .await;

        assert!(
            web_proof_result.is_ok(),
            "Generate web proof error: {:?}",
            web_proof_result.unwrap_err()
        );

        let verification_result = verify_presentation(to_presentation(&web_proof_result)).unwrap();

        assert_eq!(verification_result.sent, "GET /regular_json?are_you_sure=yes&auth=s3cret_t0ken HTTP/1.1\r\nhost: lotr-api.online\r\naccept: */*\r\naccept-encoding: identity\r\nconnection: close\r\nuser-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36\r\n\r\n");
        let regex = Regex::new("^HTTP/1\\.1 200 OK\\r\\nAccess-Control-Allow-Credentials: true\\r\\nVary: \\*\\r\\nAccess-Control-Allow-Origin: \\*\\r\\nAccess-Control-Allow-Methods: GET\\r\\nAccess-Control-Allow-Headers: host, accept, accept-encoding, connection, user-agent\\r\\nAccess-Control-Expose-Headers: host, accept, accept-encoding, connection, user-agent\\r\\nContent-Type: application/json;charset=utf-8\\r\\nDate: [A-Za-z]{3}, \\d{2} [A-Za-z]{3} \\d{4} \\d{2}:\\d{2}:\\d{2} GMT\\r\\nContent-Length: \\d+\\r\\n\\r\\n\\{\"success\":true,\"name\":\"Gandalf\",\"greeting\":\"Hello, Frodo!\"}$");
        assert!(regex.unwrap().is_match(&verification_result.recv));
        assert_eq!(verification_result.server_name, ServerName::new("lotr-api.online".to_string()));
        assert_eq!(
            verification_result.key,
            "037b48f19c139b6888fb5e383a4d72c2335186fd5858e7ae743ab4bf8e071b06e7"
        );
    }

    fn to_presentation(web_proof_result: &Result<String, Box<dyn Error>>) -> Presentation {
        let json_str = web_proof_result.as_ref().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let hex_data = parsed["data"].as_str().unwrap();
        bincode::deserialize(&hex::decode(hex_data).unwrap()).unwrap()
    }
}
