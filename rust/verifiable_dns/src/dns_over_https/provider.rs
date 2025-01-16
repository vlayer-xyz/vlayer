use super::{Query, Response};

pub trait Provider {
    type Error: Send;

    fn resolve(
        &self,
        query: &Query,
    ) -> impl std::future::Future<Output = Result<Response, Self::Error>> + Send;
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
