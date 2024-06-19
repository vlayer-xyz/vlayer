use serde::Deserialize;

use crate::json::AppJson;

#[derive(Deserialize)]
pub struct UserParams {
    name: String,
}

pub(crate) async fn hello(AppJson(params): AppJson<UserParams>) -> String {
    format!("Hello, {}!", params.name).into()
}
