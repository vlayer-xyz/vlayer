use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing::post,
    Router,
};
use axum_jrpc::{error::JsonRpcError, Id, JrpcResult, JsonRpcExtractor, JsonRpcResponse};
use serde::Serialize;
use server_utils::{init_trace_layer, RequestIdLayer};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{
    config::Config,
    handlers::{
        v_call::{self, v_call},
        v_get_proof_receipt::{self, v_get_proof_receipt},
        v_versions::{self, v_versions},
        QueryParams, SharedState,
    },
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

fn to_jrpc_result<R, E>(req_id: Id, result: Result<R, E>) -> JrpcResult
where
    R: Serialize,
    E: Into<JsonRpcError>,
{
    Ok(match result {
        Ok(result) => JsonRpcResponse::success(req_id, result),
        Err(err) => JsonRpcResponse::error(req_id, err.into()),
    })
}

async fn handle(
    Query(query): Query<QueryParams>,
    State(state): State<SharedState>,
    req: JsonRpcExtractor,
    config: Arc<Config>,
) -> JrpcResult {
    let req_id = req.get_answer_id();
    let method = req.method();
    match method {
        "v_call" => {
            let params: v_call::Params = req.parse_params()?;
            let res = v_call(config, state, query, params).await;
            to_jrpc_result(req_id, res)
        }
        "v_getProofReceipt" => {
            let params: v_get_proof_receipt::Params = req.parse_params()?;
            let res = v_get_proof_receipt(state, params).await;
            to_jrpc_result(req_id, res)
        }
        "v_versions" => {
            let params: v_versions::Params = req.parse_params()?;
            let res = v_versions(config, params).await;
            to_jrpc_result(req_id, res)
        }
        _ => Ok(req.method_not_found(method)),
    }
}

pub fn server(cfg: Config) -> Router {
    let config = Arc::new(cfg);
    let state = SharedState::default();
    //TODO: Lets decide do we need strict CORS policy or not and update this eventually
    let cors = CorsLayer::permissive();
    Router::new()
        .route("/", post(move |query, state, req| handle(query, state, req, config)))
        .with_state(state)
        .layer(cors)
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}
