use alloy_primitives::Bytes;
use web_proof::verifier::verify_and_parse;

pub fn main(input: Bytes) {
    let web_proof_json = std::str::from_utf8(&input).unwrap();
    let web_proof = serde_json::from_str(web_proof_json).unwrap();
    verify_and_parse(web_proof).unwrap();
}
