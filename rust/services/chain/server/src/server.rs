use std::{future::Future, pin::Pin, sync::Arc};

use axum::{extract::State, response::IntoResponse, routing::post, Router};
use axum_jrpc::JsonRpcExtractor;
use chain_db::ChainDb;
use parking_lot::RwLock;
use server_utils::{init_trace_layer, route, RequestIdLayer};

use crate::handlers::v_chain::v_chain;

async fn handle_jrpc(
    State(chain_db): State<Arc<RwLock<ChainDb>>>,
    request: JsonRpcExtractor,
) -> impl IntoResponse {
    let v_chain_handler = move |params| -> Pin<Box<dyn Future<Output = _> + Send>> {
        let chain_db = chain_db.clone();
        Box::pin(v_chain(chain_db, params))
    };
    route(request, "v_chain", v_chain_handler).await
}

pub fn server(chain_db: ChainDb) -> Router {
    let chain_db = Arc::new(RwLock::new(chain_db));
    Router::new()
        .route("/", post(handle_jrpc))
        .with_state(chain_db)
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}
