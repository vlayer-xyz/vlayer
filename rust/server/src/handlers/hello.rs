use serde::{Deserialize, Serialize};

use crate::json::AppJson;

#[derive(Deserialize, Serialize)]
pub struct UserParams {
    name: String,
}

impl From<&str> for UserParams {
    fn from(name: &str) -> Self {
        UserParams {
            name: name.to_string(),
        }
    }
}

pub(crate) async fn hello(AppJson(params): AppJson<UserParams>) -> String {
    format!("Hello, {}!", params.name).into()
}
