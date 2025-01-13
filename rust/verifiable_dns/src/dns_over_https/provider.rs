use super::{Query, Response};

pub trait Provider {
    fn resolve(&self, query: &Query) -> Option<Response>;
}
