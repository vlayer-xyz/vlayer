use serde::{Deserialize, Serialize};

use crate::json::AppJson;

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
pub(crate) async fn hello(AppJson(params): AppJson<UserParams>) -> String {
    format!("Hello, {}!", params.name).into()
}
