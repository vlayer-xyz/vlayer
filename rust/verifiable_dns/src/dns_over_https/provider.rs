use std::time::Duration;

use reqwest::{header::ACCEPT, Client};

use super::{Query, Response};
use crate::dns_over_https::MIME_DNS_JSON_CONTENT_TYPE;

pub trait Provider {
    async fn resolve(&self, query: &Query) -> Option<Response>;
}

#[derive(Clone)]
pub struct ExternalProvider {}

impl Provider for ExternalProvider {
    async fn resolve(&self, query: &Query) -> Option<Response> {
        let response = Client::new()
            .get("https://8.8.8.8/resolve")
            .header(ACCEPT, MIME_DNS_JSON_CONTENT_TYPE)
            .query(&query)
            .timeout(Duration::from_secs(2))
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;

        serde_json::from_str(&response).ok()?
    }
}

impl ExternalProvider {
    pub const fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn fetches_record_from_google_doh() {
        let provider = ExternalProvider::new();
        let query = "vlayer.xyz".into();

        let result = provider.resolve(&query).await.unwrap();

        assert_eq!(result.question.len(), 1);
        assert_eq!(result.question[0], query);

        assert!(!result.answer.unwrap().is_empty());
    }
}
