use crate::verifiable_dns::record::Response;

pub trait Provider {
    fn resolve(&self, query: &str) -> Option<Response>;
}
