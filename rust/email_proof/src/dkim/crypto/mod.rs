mod hash;

use hash::HashAlgorithm;

const TAG_VALUE_SEPARATOR: &str = "-";

#[derive(Debug, PartialEq)]
pub struct SignatureScheme {
    hash_algorithm: HashAlgorithm,
    cryptographic_algorithm: (),
}

impl TryFrom<&str> for SignatureScheme {
    type Error = &'static str;

    fn try_from(tag_a_value: &str) -> Result<Self, Self::Error> {
        let (_k, h) = tag_a_value
            .split_once(TAG_VALUE_SEPARATOR)
            .ok_or("Invalid tag \"a\" format")?;

        Ok(Self {
            hash_algorithm: h.try_into()?,
            cryptographic_algorithm: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod signature_scheme {
        use super::*;

        mod from_tag_a_value {
            use super::*;

            #[test]
            fn succeeds_from_valid_a_tag() {
                let a_tag = "rsa-sha256";
                let expected_scheme = SignatureScheme {
                    hash_algorithm: HashAlgorithm::Sha256,
                    cryptographic_algorithm: (),
                };
                assert_eq!(SignatureScheme::try_from(a_tag), Ok(expected_scheme));
            }

            #[test]
            fn fails_for_empty_tag() {
                let expected_err = Err("Invalid tag \"a\" format");
                assert_eq!(SignatureScheme::try_from(""), expected_err);
            }
            #[test]
            fn fails_for_invalid_tag_format() {
                let expected_err = Err("Invalid tag \"a\" format");

                assert_eq!(SignatureScheme::try_from(""), expected_err);
                assert_eq!(SignatureScheme::try_from("rsasha256"), expected_err);
            }

            #[test]
            fn fails_for_unsupported_schemes() {
                assert!(SignatureScheme::try_from("rsa-sha1").is_err());
            }
        }
    }
}
