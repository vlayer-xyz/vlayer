use risc0_zkvm::guest::env;
use web_proof::web_proof::WebProof;

fn main() {
    let input: WebProof = env::read();
    let output = web_proof::verifier::verify_and_parse(input).unwrap().abi_encode();
    env::commit(&output);
}
