use crate::handlers::hello::hello;
use axum::{routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/hello", get(hello))
}

#[cfg(test)]
mod tests {
    use core::str;

    use super::app;
    use axum::{
        body::Body,
        http::{Request, Response, StatusCode},
        Router,
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn get(app: Router, url: &str) -> anyhow::Result<Response<Body>> {
        let request = Request::get(url).body(Body::empty())?;
        Ok(app.oneshot(request).await?)
    }

    async fn body_to_string(body: Body) -> anyhow::Result<String> {
        let body_bytes = body.collect().await?.to_bytes();
        Ok(String::from_utf8(body_bytes.to_vec())?)
    }

    #[tokio::test]
    async fn hello() -> anyhow::Result<()> {
        let app = app();

        let response = get(app, "/hello").await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_to_string(response.into_body()).await?, "Hello, World!");

        Ok(())
    }
}
