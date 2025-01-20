use std::time::Duration;

use reqwest::{header::ACCEPT, Client, RequestBuilder};

use super::{Query, Response, MIME_DNS_JSON_CONTENT_TYPE};

pub trait Provider {
    type Error: Send;

    fn resolve(
        &self,
        query: &Query,
    ) -> impl std::future::Future<Output = Result<Response, Self::Error>> + Send;
}

const GOOGLE_BASE_URL: &str = "https://8.8.8.8/resolve";
const DNS_SB_BASE_URL: &str = "https://185.222.222.222/dns-query";

#[derive(Clone)]
pub struct ExternalProvider {
    base_url: &'static str,
}

#[derive(thiserror::Error, Debug)]
pub enum ExternalProviderError {
    #[error("Failed to resolve query: {0}")]
    ResolveError(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
}

impl Provider for ExternalProvider {
    type Error = ExternalProviderError;

    async fn resolve(&self, query: &Query) -> Result<Response, Self::Error> {
        let response = self.client().query(&query).send().await?.text().await?;

        Ok(serde_json::from_str(&response)?)
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

    fn client(&self) -> RequestBuilder {
        Client::new()
            .get(self.base_url)
            .header(ACCEPT, MIME_DNS_JSON_CONTENT_TYPE)
            .timeout(Duration::from_secs(2))
    }
}

#[cfg(test)]
pub(crate) mod test_utils {
    use super::*;

    #[derive(Default)]
    pub struct MockProvider(Option<Response>);

    impl MockProvider {
        pub const fn new(response: Response) -> Self {
            Self(Some(response))
        }
    }

    impl Provider for MockProvider {
        type Error = ();

        async fn resolve(&self, query: &Query) -> Result<Response, Self::Error> {
            self.0
                .clone()
                .map(|mut r| {
                    r.question.push(query.clone());
                    r
                })
                .ok_or(())
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
