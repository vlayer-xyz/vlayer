#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod integration_tests {
    use web_prover::{generate_web_proof, notarize};

    #[tokio::test]
    async fn test_notarize() {
        let notarization_result = Box::pin(notarize(
            "127.0.0.1",
            7047,
            "lotr-api.online",
            "127.0.0.1",
            3011,
            "/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
        ))
        .await;

        assert!(
            notarization_result.is_ok(),
            "Notarization error: {:?}",
            notarization_result.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_generate_web_proof() {
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
    }
}
