use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Request, Response},
    Router,
};
use http_body_util::BodyExt;
use mime::APPLICATION_JSON;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::to_string;
use tower::ServiceExt;

pub(crate) mod anvil;

pub(crate) async fn post<T>(app: Router, url: &str, body: &T) -> anyhow::Result<Response<Body>>
where
    T: Serialize,
{
    let request = Request::post(url)
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(Body::from(to_string(body)?))?;
    Ok(app.oneshot(request).await?)
}

pub(crate) async fn body_to_string(body: Body) -> anyhow::Result<String> {
    let body_bytes = body.collect().await?.to_bytes();
    Ok(String::from_utf8(body_bytes.to_vec())?)
}

pub(crate) async fn body_to_json<T: DeserializeOwned>(body: Body) -> anyhow::Result<T> {
    let body_bytes = body.collect().await?.to_bytes();
    let deserialized = serde_json::from_slice(&body_bytes)?;
    Ok(deserialized)
}
