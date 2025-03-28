use std::collections::HashMap;

use web_prover::{generate_web_proof, NotaryConfig};

#[tokio::main]
pub async fn main() {
    let presentation = Box::pin(generate_web_proof(
        NotaryConfig::new("127.0.0.1".into(), 7047, "".into(), false),
        "lotr-api.online",
        "127.0.0.1",
        3011,
        "/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
        HashMap::new(),
    ))
    .await
    .unwrap();

    println!("{presentation}");
}
