use crate::{Provider, Query, Response, common::types::Timestamp, verifiable_dns::time::Now};

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

pub(crate) struct MockClock<const T: Timestamp>;

impl<const T: Timestamp> Now for MockClock<T> {
    fn now() -> Timestamp {
        T
    }
}
