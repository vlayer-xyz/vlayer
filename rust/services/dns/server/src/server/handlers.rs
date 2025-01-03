use std::collections::HashMap;

use axum::routing::MethodRouter;

pub(super) fn handlers() -> HashMap<&'static str, MethodRouter> {
    [].into()
}
