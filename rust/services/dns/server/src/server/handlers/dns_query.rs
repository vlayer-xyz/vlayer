mod types;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::{MethodRouter, get},
};
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::debug;
use types::{Params, ServerResponse};
use verifiable_dns::{MIME_DNS_JSON_CONTENT_TYPE, Provider};

use crate::server::AppState;

pub fn handler() -> MethodRouter<AppState> {
    get(dns_query_handler)
        .route_layer(ValidateRequestHeaderLayer::accept(MIME_DNS_JSON_CONTENT_TYPE))
}

async fn dns_query_handler(
    State(state): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    debug!("Querying for {:?}", &params);
    let result = ServerResponse(state.vdns_resolver.resolve(&params.into()).await.unwrap());
    debug!("Responding with: {:?}", &result);
    result
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{HeaderName, Response, StatusCode, header::ACCEPT},
    };
    use server_utils::get;

    use super::*;
    use crate::server::{handlers::DNS_QUERY_PATH, test_helpers::app};

    const DEFAULT_PARAMS: [(&str, &str); 2] =
        [("name", "google._domainkey.vlayer.xyz"), ("type", "txt")];

    async fn dns_query() -> Response<Body> {
        run_dns_query(None, None).await
    }

    async fn dns_query_with_headers(headers: &[(&HeaderName, &str)]) -> Response<Body> {
        run_dns_query(Some(headers), None).await
    }

    async fn dns_query_with_params(params: &[(&str, &str)]) -> Response<Body> {
        run_dns_query(None, Some(params)).await
    }

    async fn run_dns_query(
        headers: Option<&[(&HeaderName, &str)]>,
        params: Option<&[(&str, &str)]>,
    ) -> Response<Body> {
        let app = app();

        let headers = headers.unwrap_or(&[(&ACCEPT, MIME_DNS_JSON_CONTENT_TYPE)]);
        let params = params.unwrap_or(&DEFAULT_PARAMS);

        get(app, DNS_QUERY_PATH, headers, params).await
    }

    #[tokio::test]
    async fn serves_doh_get_request() {
        let response = dns_query().await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    mod response {
        use axum::http::header::CONTENT_TYPE;

        use super::*;

        #[tokio::test]
        async fn response_is_of_application_dns_json_mime_type() {
            let response = dns_query().await;
            let (_h, content_type) = response
                .headers()
                .iter()
                .find(|h| h.0 == CONTENT_TYPE)
                .unwrap();

            let content_type = content_type.to_str().unwrap();

            assert_eq!(content_type, MIME_DNS_JSON_CONTENT_TYPE);
        }
    }

    mod query {
        use super::*;

        #[tokio::test]
        async fn fails_for_non_dns_json_response_type() {
            let response = dns_query_with_headers(&[(&ACCEPT, "application/json")]).await;
            assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
        }

        #[tokio::test]
        async fn fails_for_invalid_params() {
            assert!(
                dns_query_with_params(&DEFAULT_PARAMS[0..1])
                    .await
                    .status()
                    .is_client_error()
            );

            assert!(
                dns_query_with_params(&DEFAULT_PARAMS[1..])
                    .await
                    .status()
                    .is_client_error()
            );
        }
    }
}
