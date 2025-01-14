use std::time::Duration;

use reqwest::{header::ACCEPT, Client};

use super::{Query, Response};
use crate::dns_over_https::MIME_DNS_JSON_CONTENT_TYPE;

pub trait Provider {
    async fn resolve(&self, query: &Query) -> Option<Response>;
}

const GOOGLE_BASE_URL: &str = "https://8.8.8.8/resolve";

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
}

#[cfg(test)]
pub(crate) mod test_utils {
    use super::*;

    #[derive(Default)]
    pub struct MockProvider(Option<Response>);

    impl MockProvider {
        pub fn new(response: Response) -> Self {
            Self(Some(response))
        }
    }

    impl Provider for MockProvider {
        async fn resolve(&self, query: &Query) -> Option<Response> {
            self.0.clone().map(|mut r| {
                r.question.push(query.clone());
                r
            })
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
}
