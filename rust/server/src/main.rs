mod app;
mod error;
mod handlers;
mod json;
mod layers;
mod trace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::server().await
}
