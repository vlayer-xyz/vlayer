mod request_id;
mod trace;

pub use request_id::{RequestId, RequestIdLayer};
use tower_http::cors::CorsLayer;
pub use trace::init_trace_layer;

pub fn cors() -> CorsLayer {
    //TODO: Lets decide do we need strict CORS policy or not and update this eventually
    CorsLayer::permissive()
}
