use super::{Query, Response};

pub trait Provider {
    type Error: Send;

    fn resolve(
        &self,
        query: &Query,
    ) -> impl std::future::Future<Output = Result<Response, Self::Error>> + Send;
}
