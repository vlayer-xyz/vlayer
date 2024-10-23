use std::{future::Future, pin::Pin, sync::Arc};

use axum::{response::IntoResponse, routing::post, Extension, Router};
use axum_jrpc::JsonRpcExtractor;
use chain_db::ChainDb;
use parking_lot::RwLock;
use server_utils::route;

use crate::handlers::v_chain::v_chain;
pub use crate::{handlers::v_chain::ChainProof, mock::ChainProofServerMock};

async fn handle_jrpc(
    Extension(chain_db): Extension<Arc<RwLock<ChainDb>>>,
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
        .layer(Extension(chain_db))
}
