use serde::{Deserialize, Serialize};

use crate::json::Json;

#[derive(Deserialize, Serialize)]
pub struct UserParams {
    name: String,
}

#[cfg(test)]
impl UserParams {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
pub(crate) async fn hello(Json(params): Json<UserParams>) -> String {
    format!("Hello, {}!", params.name)
}
