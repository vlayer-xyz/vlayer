use crate::dns_over_https::{Provider as DoHProvider, Query, Response};

#[derive(Clone)]
pub(crate) struct Resolver {}

impl Resolver {
    pub const fn new() -> Self {
        Self {}
    }
}

impl DoHProvider for Resolver {
    fn resolve(&self, _query: &Query) -> Option<Response> {
        Some(Default::default())
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
}
