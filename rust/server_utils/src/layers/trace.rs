use axum::http::Request;
use tower_http::trace::{HttpMakeClassifier, TraceLayer};
use tracing::{Span, info_span};

use crate::layers::request_id::request_id;

pub fn init_trace_layer<B>() -> TraceLayer<HttpMakeClassifier, impl Fn(&Request<B>) -> Span + Clone>
{
    TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
        info_span!("http", method = ?request.method(), uri = ?request.uri(), id = request_id(request))
    })
}
