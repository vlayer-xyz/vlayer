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

#[derive(Debug, PartialEq)]
pub enum HashAlgorithm {
    Sha256,
}

impl TryFrom<&str> for HashAlgorithm {
    type Error = &'static str;

    fn try_from(sig_a_tag_h: &str) -> Result<Self, Self::Error> {
        match sig_a_tag_h {
            "sha256" => Ok(Self::Sha256),
            "sha1" => Err("Unsupported signature hashing algorithm: sha1"),
            _ => Err("Invalid header sig-a-tag-h value"),
        }
    }
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

    mod hash_algorithm {
        use super::*;

        mod from_tag_a {
            use lazy_static::lazy_static;

            use super::*;

            const INVALID_TAG_A_VALUE_ERR: std::result::Result<HashAlgorithm, &'static str> =
                Err("Invalid header sig-a-tag-h value");

            #[test]
            fn succeeds_for_sha256() {
                let tag_value = "sha256";
                assert_eq!(HashAlgorithm::try_from(tag_value), Ok(HashAlgorithm::Sha256))
            }

            #[test]
            fn disable_support_for_sha1() {
                let tag_value = "sha1";
                assert_eq!(
                    HashAlgorithm::try_from(tag_value),
                    Err("Unsupported signature hashing algorithm: sha1")
                )
            }

            #[test]
            fn fails_for_upper_case() {
                let tag_value = "sha256".to_uppercase();
                assert_eq!(HashAlgorithm::try_from(tag_value.as_str()), INVALID_TAG_A_VALUE_ERR)
            }

            #[test]
            fn fails_when_passed_whole_a_tag() {
                let tag_value = "rsa-sha256";
                assert_eq!(HashAlgorithm::try_from(tag_value), INVALID_TAG_A_VALUE_ERR);
            }

            #[test]
            fn fails_for_empty_value() {
                let tag_value = "";
                assert_eq!(HashAlgorithm::try_from(tag_value), INVALID_TAG_A_VALUE_ERR);
            }
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
