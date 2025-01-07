use super::signer::Signer;
use crate::dns_over_https::{Provider as DoHProvider, Query, Response};

#[derive(Clone)]
pub(crate) struct Resolver {
    signer: Signer,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            signer: Signer::new(),
        }
    }
}

impl DoHProvider for Resolver {
    fn resolve(&self, query: &Query) -> Option<Response> {
        let mut response: Response = Response::with_flags(false, true, true, false, false);
        response.question = query.clone();
        response.status = 0;

        response.signature = Some(self.signer.sign(&query.name.as_str()));

        Some(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_vlayer_default_dkim_selector() {
        let resolver = Resolver::new();
        let result = resolver.resolve(&"google._domainkey.vlayer.xyz".into());
        assert!(result.is_some());
    }

    #[test]
    fn question_field_equals_to_question() {
        let resolver = Resolver::new();
        let query = "google._domainkey.vlayer.xyz".into();
        let result = resolver.resolve(&query).unwrap();

        assert_eq!(result.question, query);
    }

    #[test]
    fn status_code_is_successful() {
        let resolver = Resolver::new();
        let query = "google._domainkey.vlayer.xyz".into();
        let result = resolver.resolve(&query).unwrap();

        assert_eq!(result.status, 0);
    }

    #[test]
    fn flags_are_set() {
        let resolver = Resolver::new();
        let query = "google._domainkey.vlayer.xyz".into();
        let result = resolver.resolve(&query).unwrap();

        // Default values by CF
        assert!(!result.truncated);
        assert!(result.recursive_desired);
        assert!(result.recursion_available);
        assert!(!result.verified_with_dnssec);
    }
}
