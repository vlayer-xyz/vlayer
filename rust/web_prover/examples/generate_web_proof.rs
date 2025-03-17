use web_prover::{create_presentation, notarize};

#[tokio::main]
pub async fn main() {
    let notarization_result = Box::pin(notarize(
        "127.0.0.1",
        7047,
        "lotr-api.online",
        "127.0.0.1",
        3011,
        "/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
    ))
    .await
    .unwrap();

    let presentation = create_presentation(notarization_result.0, notarization_result.1)
        .await
        .unwrap();

    println!("{}", presentation);
}
