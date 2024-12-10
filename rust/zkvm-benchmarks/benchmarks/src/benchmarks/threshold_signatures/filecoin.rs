use crate::{with_fixture, Benchmark};

pub fn benchmarks() -> Vec<Benchmark> {
    vec![Benchmark::new("bls", with_fixture!(bls::setup(), bls::verify), 35_000_000)]
}

const MESSAGE: &'static str = "ala ma kota";

mod bls {

    use bls_signatures::{aggregate, hash, verify as verify_bls, PrivateKey, PublicKey, Signature};
    use rand::thread_rng;

    use super::MESSAGE;

    const MIN_SIGNERS: usize = 3;

    pub(super) fn setup() -> (Signature, Vec<PublicKey>) {
        let mut rng = thread_rng();
        let max_signers = 5;

        let keys: Vec<_> = (0..max_signers)
            .map(|_| PrivateKey::generate(&mut rng))
            .collect();
        let pub_keys: Vec<PublicKey> = keys.iter().map(|key| key.public_key()).collect();

        let signatures: Vec<Signature> = keys
            .iter()
            .take(MIN_SIGNERS)
            .map(|key| key.sign(MESSAGE.as_bytes()))
            .collect();

        let aggregate = aggregate(&signatures).unwrap();

        (aggregate, pub_keys)
    }

    pub(super) fn verify((signature, pub_keys): (Signature, Vec<PublicKey>)) {
        let msg_hash = hash(MESSAGE.as_bytes());
        let (hashes, pub_keys): (Vec<_>, Vec<_>) = (0..MIN_SIGNERS)
            .map(|_| msg_hash.clone())
            .zip(pub_keys)
            .unzip();

        verify_bls(&signature, &hashes, &pub_keys);
    }
}
