use rsa::{
    RsaPrivateKey, RsaPublicKey, pkcs1v15,
    pkcs1v15::{Signature, VerifyingKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    signature::{Keypair, Signer, Verifier},
};
use sha2::Sha256;

use crate::Benchmark;

const RSA_2048_PRIV_PEM: &str = include_str!("../../../assets/rsa2048-priv.pem");
const RSA_3072_PRIV_PEM: &str = include_str!("../../../assets/rsa3072-priv.pem");
const RSA_2048_PUB_PEM: &str = include_str!("../../../assets/rsa2048-pub.pem");

const EMAIL: &str = include_str!("../../../assets/email.eml");

struct RsaBenchmarkInputs {
    verifying_key: VerifyingKey<Sha256>,
    signature: Signature,
    msg: Vec<u8>,
}

impl RsaBenchmarkInputs {
    fn new(private_key_pem: &str, msg: &[u8]) -> Self {
        let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem).unwrap();
        let signing_key = pkcs1v15::SigningKey::<Sha256>::new(private_key);
        let signature = signing_key.sign(msg);
        let verifying_key = signing_key.verifying_key();

        Self {
            msg: msg.into(),
            verifying_key,
            signature,
        }
    }
}

fn rsa_public_key_from_string(public_key_pem: &str) {
    RsaPublicKey::from_public_key_pem(public_key_pem).unwrap();
}

fn rsa_verification(input: RsaBenchmarkInputs) {
    input
        .verifying_key
        .verify(input.msg.as_slice(), &input.signature)
        .unwrap()
}

pub fn benchmarks() -> Vec<Benchmark> {
    let bench_2048 = RsaBenchmarkInputs::new(RSA_2048_PRIV_PEM, EMAIL.as_bytes());
    let bench_3072 = RsaBenchmarkInputs::new(RSA_3072_PRIV_PEM, EMAIL.as_bytes());

    vec![
        Benchmark::new("rsa_2048_verification", move || rsa_verification(bench_2048), 9_971_442),
        Benchmark::new("rsa_3072_verification", move || rsa_verification(bench_3072), 21_917_991),
        Benchmark::new(
            "rsa_2048_public_key",
            move || rsa_public_key_from_string(RSA_2048_PUB_PEM),
            53_576,
        ),
    ]
}
