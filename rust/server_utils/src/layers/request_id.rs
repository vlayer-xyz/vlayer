use axum::http::Request;
pub use tower_request_id::{RequestId, RequestIdLayer};

pub fn request_id<B>(request: &Request<B>) -> String {
    request
        .extensions()
        .get::<RequestId>()
        .map_or_else(|| "unknown".into(), ToString::to_string)
}
