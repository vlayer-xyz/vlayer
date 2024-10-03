#![allow(unused)]
use ring::{self, digest::SHA256_OUTPUT_LEN};

type Sha256Output = [u8; SHA256_OUTPUT_LEN];

pub trait Hashable {
    fn hashable_bytes(&self) -> &[u8];

    fn hash_with(&self, algorithm: HashAlgorithm) -> Digest {
        let bytes = self.hashable_bytes();
        algorithm.hash(bytes)
    }
}

pub enum HashAlgorithm {
    Sha256,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Digest {
    Sha256(Sha256Output),
}

impl HashAlgorithm {
    fn hash(&self, bytes: &[u8]) -> Digest {
        match &self {
            HashAlgorithm::Sha256 => Digest::Sha256(sha256(bytes)),
        }
    }
}

fn sha256(bytes: &[u8]) -> Sha256Output {
    let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
    hasher.update(bytes);
    let digest = hasher.finish();

    digest.as_ref().try_into().expect("Invalid digest size")
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Hashable for &str {
        fn hashable_bytes(&self) -> &[u8] {
            self.as_bytes()
        }
    }

    mod hash {
        use hex_literal::hex;

        use super::*;

        #[test]
        fn for_sha256_returns_sha256_digest() {
            let hash_algorithm = HashAlgorithm::Sha256;
            let data = "ala ma kota";

            matches!(hash_algorithm.hash(data.as_bytes()), Digest::Sha256(_));
            matches!(data.hash_with(hash_algorithm), Digest::Sha256(_));
        }

        #[test]
        fn for_sha256_returns_sha256_of_provided_data() {
            let hash_algorithm = HashAlgorithm::Sha256;

            assert_eq!(
                hash_algorithm.hash("ala ma kota".as_bytes()),
                Digest::Sha256(hex!(
                    "c623e3ee2d7fa2c770f19cace523191cf92f1d59b0678bbbb1825817c9a61575"
                ))
            );
        }
    }
}
