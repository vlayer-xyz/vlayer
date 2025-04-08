mod dns_query;

use std::collections::HashMap;

use axum::routing::{MethodRouter, get};

use super::AppState;

const DNS_QUERY_PATH: &str = "/dns-query";
const HEALTH_PATH: &str = "/health";

pub(super) fn handlers() -> HashMap<&'static str, MethodRouter<AppState>> {
    [(DNS_QUERY_PATH, dns_query::handler()), (HEALTH_PATH, health_handler())].into()
}

fn health_handler() -> MethodRouter<AppState> {
    get(|| async { "OK" })
}
