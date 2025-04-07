use std::collections::HashMap;

use web_prover::{generate_web_proof, NotarizeParamsBuilder, NotaryConfig};

#[tokio::main]
pub async fn main() {
    let presentation = Box::pin(generate_web_proof(
        NotarizeParamsBuilder::default()
            .notary_config(NotaryConfig::new("127.0.0.1".into(), 7047, "".into(), false))
            .server_domain("lotr-api.online")
            .server_host("127.0.0.1")
            .server_port(3011_u16)
            .uri("/regular_json?are_you_sure=yes&auth=s3cret_t0ken")
            .build()
            .unwrap(),
    ))
    .await
    .unwrap();

    println!("{presentation}");
}
