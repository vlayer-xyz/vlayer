mod types;

use std::iter::once;

use axum::{
    extract::{Query, State},
    http::header::AUTHORIZATION,
    response::IntoResponse,
    routing::{MethodRouter, get},
};
use server_utils::jwt::{Claims, axum::ClaimsExtractor};
use tower_http::{
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::debug;
use types::{Params, ServerResponse};
use verifiable_dns::{MIME_DNS_JSON_CONTENT_TYPE, Provider};

use crate::server::{AppState, Config};

#[allow(clippy::needless_pass_by_value)]
pub fn handler(config: Config) -> MethodRouter<AppState> {
    let handler = if config.jwt_config.is_some() {
        get(dns_query_handler_with_auth)
    } else {
        get(dns_query_handler)
    };
    get(handler)
        .route_layer(ValidateRequestHeaderLayer::accept(MIME_DNS_JSON_CONTENT_TYPE))
        .route_layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
}

async fn dns_query_handler_with_auth(
    _claims: ClaimsExtractor<Claims>,
    state: State<AppState>,
    params: Query<Params>,
) -> impl IntoResponse {
    dns_query_handler(state, params).await
}

#[allow(clippy::unwrap_used)]
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
        Router,
        body::Body,
        http::{HeaderName, Response, StatusCode, header::ACCEPT},
    };
    use server_utils::get;

    use super::*;
    use crate::server::{handlers::DNS_QUERY_PATH, test_helpers::app};

    const DEFAULT_PARAMS: [(&str, &str); 2] =
        [("name", "google._domainkey.vlayer.xyz"), ("type", "txt")];

    async fn dns_query() -> Response<Body> {
        run_dns_query(app(), None, None).await
    }

    async fn dns_query_with_headers(headers: &[(&HeaderName, &str)]) -> Response<Body> {
        run_dns_query(app(), Some(headers), None).await
    }

    async fn dns_query_with_params(params: &[(&str, &str)]) -> Response<Body> {
        run_dns_query(app(), None, Some(params)).await
    }

    async fn run_dns_query(
        app: Router,
        headers: Option<&[(&HeaderName, &str)]>,
        params: Option<&[(&str, &str)]>,
    ) -> Response<Body> {
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

    mod jwt {
        use server_utils::jwt::test_helpers::{JWT_SECRET, TokenArgs, token as test_token};

        use super::*;
        use crate::server::test_helpers::app_with_jwt_auth;

        fn token(invalid_after: i64, subject: &str) -> String {
            test_token(&TokenArgs {
                secret: JWT_SECRET,
                host: None,
                port: None,
                invalid_after,
                subject,
                environment: None,
            })
        }

        async fn run_dns_query_with_token(token: String) -> Response<Body> {
            let auth = format!("Bearer {token}");
            let headers = &[(&AUTHORIZATION, auth.as_str()), (&ACCEPT, MIME_DNS_JSON_CONTENT_TYPE)];
            run_dns_query(app_with_jwt_auth(), Some(headers), None).await
        }

        #[tokio::test]
        async fn accepts_requests_with_valid_token() {
            assert_eq!(run_dns_query_with_token(token(60, "1234")).await.status(), StatusCode::OK)
        }

        #[tokio::test]
        async fn rejects_requests_with_missing_token() {
            assert_eq!(
                run_dns_query(app_with_jwt_auth(), None, None)
                    .await
                    .status(),
                StatusCode::UNAUTHORIZED
            )
        }

        #[tokio::test]
        async fn rejects_requests_with_expired_token() {
            assert_eq!(
                run_dns_query_with_token(token(-120, "1234")).await.status(),
                StatusCode::UNAUTHORIZED
            )
        }

        #[tokio::test]
        async fn rejects_requests_with_tampered_with_token() {
            assert_eq!(
                run_dns_query_with_token(test_token(&TokenArgs {
                    secret: "beefdead",
                    host: None,
                    port: None,
                    invalid_after: 60,
                    subject: "1234",
                    environment: None,
                }))
                .await
                .status(),
                StatusCode::UNAUTHORIZED
            )
        }
    }
}
