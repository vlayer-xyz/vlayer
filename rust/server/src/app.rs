use crate::handlers::{call::call, hello::hello};
use crate::layers::request_id::RequestIdLayer;
use crate::layers::trace::init_trace_layer;
use axum::{routing::post, Router};

pub fn app() -> Router {
    Router::new()
        .route("/hello", post(hello))
        .route("/call", post(call))
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}

#[cfg(test)]
mod tests {
    use core::str;

    use super::app;
    use crate::error::ErrorResponse;
    use crate::handlers::hello::UserParams;
    use axum::{
        body::Body,
        http::{header::CONTENT_TYPE, Request, Response, StatusCode},
        Router,
    };
    use http_body_util::BodyExt;
    use mime::APPLICATION_JSON;
    use serde::{de::DeserializeOwned, Serialize};
    use serde_json::to_string;
    use tower::ServiceExt;

    async fn post<T>(app: Router, url: &str, body: &T) -> anyhow::Result<Response<Body>>
    where
        T: Serialize,
    {
        let request = Request::post(url)
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
            .body(Body::from(to_string(body)?))?;
        Ok(app.oneshot(request).await?)
    }

    async fn body_to_string(body: Body) -> anyhow::Result<String> {
        let body_bytes = body.collect().await?.to_bytes();
        Ok(String::from_utf8(body_bytes.to_vec())?)
    }

    async fn body_to_json<T: DeserializeOwned>(body: Body) -> anyhow::Result<T> {
        let body_bytes = body.collect().await?.to_bytes();
        let deserialized = serde_json::from_slice(&body_bytes)?;
        Ok(deserialized)
    }

    #[tokio::test]
    async fn hello() -> anyhow::Result<()> {
        let app = app();

        let user = UserParams::new("Name");
        let response = post(app, "/hello", &user).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_to_string(response.into_body()).await?, "Hello, Name!");

        Ok(())
    }

    #[tokio::test]
    async fn not_found() -> anyhow::Result<()> {
        let app = app();

        let response = post(app, "/non_existent", &()).await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(body_to_string(response.into_body()).await?, "");

        Ok(())
    }

    #[tokio::test]
    async fn json_rejection() -> anyhow::Result<()> {
        let app = app();

        let response = post(app, "/hello", &()).await?;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            body_to_json::<ErrorResponse>(response.into_body()).await?,
            ErrorResponse::new("Failed to deserialize the JSON body into the target type: invalid type: null, expected struct UserParams at line 1 column 4"),
        );

        Ok(())
    }
}
