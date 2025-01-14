use super::{Query, Response};

pub trait Provider {
    async fn resolve(&self, query: &Query) -> Option<Response>;
}
