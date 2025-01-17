use std::time::Duration;

use reqwest::{header::ACCEPT, Client};

use crate::{Provider, Query, Response, MIME_DNS_JSON_CONTENT_TYPE};

const GOOGLE_BASE_URL: &str = "https://8.8.8.8/resolve";
const DNS_SB_BASE_URL: &str = "https://185.222.222.222/dns-query";

#[derive(Clone)]
pub struct ExternalProvider {
    base_url: &'static str,
}

impl Provider for ExternalProvider {
    async fn resolve(&self, query: &Query) -> Option<Response> {
        let response = Client::new()
            .get(self.base_url)
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
    pub const fn google_provider() -> Self {
        Self {
            base_url: GOOGLE_BASE_URL,
        }
    }

    pub const fn dns_sb_provider() -> Self {
        Self {
            base_url: DNS_SB_BASE_URL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetches_record_from_google_doh() {
        let provider = ExternalProvider::google_provider();
        let query = "vlayer.xyz".into();

        let result = provider.resolve(&query).await.unwrap();

        assert_eq!(result.question.len(), 1);
        assert_eq!(result.question[0], query);

        assert!(!result.answer.unwrap().is_empty());
    }

    #[tokio::test]
    async fn fetches_record_from_dnssb_doh() {
        let provider = ExternalProvider::dns_sb_provider();
        let query = "vlayer.xyz".into();

        let result = provider.resolve(&query).await.unwrap();

        assert_eq!(result.question.len(), 1);
        assert_eq!(result.question[0], query);
        dbg!(&result);
        assert!(!result.answer.unwrap().is_empty());
    }

    #[tokio::test]
    async fn providers_fetch_dkim_record() {
        let providers = [ExternalProvider::dns_sb_provider(), ExternalProvider::google_provider()];
        let query = "google._domainkey.vlayer.xyz".into();

        for provider in providers {
            let result = provider.resolve(&query).await.unwrap();

            assert_eq!(result.question.len(), 1);
            assert_eq!(result.question[0], query);

            assert!(!result.answer.unwrap().is_empty());
        }
    }
}
