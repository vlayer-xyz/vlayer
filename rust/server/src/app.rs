use crate::handlers::hello::hello;
use crate::layers::request_id::RequestIdLayer;
use crate::layers::trace::init_trace_layer;
use axum::{routing::post, Router};

pub fn app() -> Router {
    Router::new()
        .route("/hello", post(hello))
        .layer(init_trace_layer())
        // NOTE: it should be added after the Trace layer
        .layer(RequestIdLayer)
}

#[cfg(test)]
mod tests {
    use core::str;

    use super::app;
    use crate::handlers::hello::UserParams;
    use axum::{
        body::Body,
        http::{header::CONTENT_TYPE, Request, Response, StatusCode},
        Router,
    };
    use http_body_util::BodyExt;
    use serde::Serialize;
    use serde_json::to_string;
    use tower::ServiceExt;

    async fn post<T>(app: Router, url: &str, body: &T) -> anyhow::Result<Response<Body>>
    where
        T: Serialize,
    {
        let request = Request::post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(to_string(body)?))?;
        Ok(app.oneshot(request).await?)
    }

    async fn body_to_string(body: Body) -> anyhow::Result<String> {
        let body_bytes = body.collect().await?.to_bytes();
        Ok(String::from_utf8(body_bytes.to_vec())?)
    }

    #[tokio::test]
    async fn hello() -> anyhow::Result<()> {
        let app = app();

        let user: UserParams = "Name".into();
        let response = post(app, "/hello", &user).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_to_string(response.into_body()).await?, "Hello, Name!");

        Ok(())
    }
}
