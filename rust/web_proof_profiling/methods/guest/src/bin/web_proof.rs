use web_proof::{types::WebProof, verifier::_verify_proof, fixtures::{tls_proof_example, pub_key}};

use risc0_zkvm::guest::env;

fn main() {
    // we must call env::read(), otherwise guest doesn't compile
    let _x: u8 = env::read();

    _verify_proof(WebProof {
        tls_proof: tls_proof_example(),
        notary_pub_key: pub_key(),
    }).unwrap();
    
}