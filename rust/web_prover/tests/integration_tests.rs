#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod integration_tests {
    use web_prover::notarize;

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
}
