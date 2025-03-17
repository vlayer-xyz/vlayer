use web_prover::generate_web_proof;

#[tokio::main]
pub async fn main() {
    let presentation = Box::pin(generate_web_proof(
        "127.0.0.1",
        7047,
        "lotr-api.online",
        "127.0.0.1",
        3011,
        "/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
    ))
    .await
    .unwrap();

    println!("{}", presentation);
}
