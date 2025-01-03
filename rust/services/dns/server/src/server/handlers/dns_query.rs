mod types;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::{get, MethodRouter},
};
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::debug;
use types::Params;

use crate::server::AppState;

const ACCEPTED_RESPONSE_TYPE: &str = "application/dns-json";
pub fn handler() -> MethodRouter<AppState> {
    get(dns_query_handler).route_layer(ValidateRequestHeaderLayer::accept(ACCEPTED_RESPONSE_TYPE))
}

async fn dns_query_handler(
    State(_state): State<AppState>,
    Query(query): Query<Params>,
) -> impl IntoResponse {
    debug!("Querying for {:?}", &query);
    ""
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{header::ACCEPT, HeaderName, Response, StatusCode},
    };
    use server_utils::get;

    use super::*;
    use crate::server::{handlers::DNS_QUERY_PATH, test_helpers::app};

    const DEFAULT_PARAMS: [(&str, &str); 2] =
        [("name", "google._domainkey.vlayer.xyz"), ("type", "txt")];

    async fn dns_query(
        headers: Option<&[(&HeaderName, &str)]>,
        params: Option<&[(&str, &str)]>,
    ) -> Response<Body> {
        let app = app();

        let headers = headers.unwrap_or(&[(&ACCEPT, ACCEPTED_RESPONSE_TYPE)]);
        let params = params.unwrap_or(&DEFAULT_PARAMS);

        get(app, DNS_QUERY_PATH, headers, params).await
    }

    #[tokio::test]
    async fn serves_doh_get_request() {
        let response = dns_query(None, None).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn fails_for_non_dns_json_response_type() {
        let response = dns_query(Some(&[(&ACCEPT, "application/json")]), None).await;
        assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
    }

    #[tokio::test]
    async fn fails_for_invalid_params() {
        assert!(dns_query(None, Some(&DEFAULT_PARAMS[0..1]))
            .await
            .status()
            .is_client_error());

        assert!(dns_query(None, Some(&DEFAULT_PARAMS[1..]))
            .await
            .status()
            .is_client_error());
    }
}
