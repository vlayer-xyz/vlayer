use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use axum_jrpc::{error::JsonRpcError, JrpcResult, JsonRpcExtractor, JsonRpcResponse};
use futures::FutureExt;
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};

type HandlerFuture<R, E> = Pin<Box<dyn Future<Output = Result<R, E>> + Send + 'static>>;
pub trait Handler<Params: DeserializeOwned>: Send + Sync + 'static {
    type Return: Serialize;
    type Error: Into<JsonRpcError>;
    fn handle(&self, params: Params) -> HandlerFuture<Self::Return, Self::Error>;
}

impl<P, R, E, F> Handler<P> for F
where
    P: DeserializeOwned,
    R: Serialize,
    E: Into<JsonRpcError>,
    F: Fn(P) -> HandlerFuture<R, E> + Send + Sync + 'static,
{
    type Error = E;
    type Return = R;

    fn handle(&self, params: P) -> HandlerFuture<R, E> {
        self(params)
    }
}

// Shared reference to an async function taking `JsonRpcExtractor` and returning `JrpcResult`
type WrappedHandler = Arc<
    dyn Fn(JsonRpcExtractor) -> Pin<Box<dyn Future<Output = JrpcResult> + Send + 'static>>
        + Send
        + Sync
        + 'static,
>;

fn wrap_handler<Params: DeserializeOwned>(handler: impl Handler<Params>) -> WrappedHandler {
    let handler = Arc::new(handler);
    Arc::new(move |request| {
        let handler = handler.clone();
        async move {
            let request_id = request.get_answer_id();
            let params = request.parse_params()?;

            Ok(match handler.handle(params).await {
                Ok(result) => JsonRpcResponse::success(request_id, result),
                Err(err) => JsonRpcResponse::error(request_id, err.into()),
            })
        }
        .boxed()
    })
}

#[derive(Default, Clone)]
pub struct Router {
    handlers: Arc<RwLock<HashMap<String, WrappedHandler>>>,
}

impl Router {
    pub fn add_handler<Params: DeserializeOwned>(
        &mut self,
        method: &str,
        handler: impl Handler<Params>,
    ) {
        let wrapped_handler = wrap_handler(handler);
        self.handlers
            .write()
            .insert(method.to_string(), wrapped_handler);
    }

    pub async fn handle_request(&self, request: JsonRpcExtractor) -> JrpcResult {
        let method = request.method();
        let handler = if let Some(handler) = self.handlers.read().get(method) {
            handler.clone() // Clone the handler not to carry read guard across await point
        } else {
            return Err(request.method_not_found(method));
        };
        handler(request).await
    }
}
