use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use block_proof::server;
use tower::ServiceExt;

async fn test_post(url: &str) -> Response<Body> {
    let app = server();
    let request = Request::post(url).body(Body::from("")).unwrap();
    app.oneshot(request).await.unwrap()
}

#[tokio::test]
async fn not_found() {
    let response = test_post("/non_existing").await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}
