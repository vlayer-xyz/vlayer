use super::{Query, Response};

pub trait Provider {
    fn resolve(&self, query: &Query) -> impl std::future::Future<Output = Option<Response>> + Send;
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
        async fn resolve(&self, query: &Query) -> Option<Response> {
            self.0.clone().map(|mut r| {
                r.question.push(query.clone());
                r
            })
        }
    }
}
