use std::process::exit;

mod cycle;
mod guest;
mod row;
mod runner;
mod tolerance;

fn main() {
    let result = runner::run();

    if let Err(err) = result {
        eprintln!("❌ Failed to run benchmarks: {err}");
        exit(1);
    }
    println!("✅ Successfully run all benchmarks");

    // setup(3);
}

// use rand::thread_rng;
// use risc0_zkvm::sha::rust_crypto::Sha256;
// use rsa::{
//     pkcs1::EncodeRsaPublicKey,
//     pkcs1v15::{Signature, SigningKey, VerifyingKey},
//     signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier},
//     RsaPrivateKey,
// };

// const MESSAGE: &'static str = "ala ma kota";

// fn setup(min_signers: usize) -> (Vec<Signature>, Vec<VerifyingKey<Sha256>>) {
//     let mut rng = thread_rng();
//     let bits = 2048;
//     let max_signers = 5;

//     let keys: Vec<_> = (0..max_signers)
//         .map(|_| RsaPrivateKey::new(&mut rng, bits).unwrap())
//         .map(|key| SigningKey::<Sha256>::new(key))
//         .collect();

//     let pub_keys: Vec<VerifyingKey<Sha256>> = keys.iter().map(|key| key.verifying_key()).collect();

//     let signatures: Vec<Signature> = keys
//         .iter()
//         .take(min_signers)
//         .map(|key| key.sign_with_rng(&mut rng, MESSAGE.as_bytes()))
//         .collect();

//     let pub_keys_enc = pub_keys
//         .iter()
//         .map(|k| k.to_pkcs1_pem(Default::default()).unwrap())
//         .collect::<Vec<_>>();

//     let sigs = signatures.iter().map(|s| s.to_bytes()).collect::<Vec<_>>();

//     dbg!(sigs);
//     dbg!(pub_keys_enc);

//     (signatures, pub_keys)
// }
