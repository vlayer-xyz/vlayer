mod dns_query;

use std::collections::HashMap;

use axum::routing::MethodRouter;

use super::AppState;

const DNS_QUERY_PATH: &str = "/dns-query";

pub(super) fn handlers() -> HashMap<&'static str, MethodRouter<AppState>> {
    [(DNS_QUERY_PATH, dns_query::handler())].into()
}
