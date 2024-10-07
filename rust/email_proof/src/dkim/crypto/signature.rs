#![allow(unused)]

#[derive(Debug, PartialEq)]
pub enum SignatureAlgorithm {
    Rsa,
}

impl TryFrom<&str> for SignatureAlgorithm {
    type Error = &'static str;

    fn try_from(sig_a_tag_k: &str) -> Result<Self, Self::Error> {
        match sig_a_tag_k {
            "rsa" => Ok(Self::Rsa),
            _ => Err("Invalid header sig-a-tag-k value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod signature_algorithm {
        use super::*;

        mod from_tag_a {
            use super::*;
            const INVALID_TAG_A_VALUE_ERR: std::result::Result<SignatureAlgorithm, &'static str> =
                Err("Invalid header sig-a-tag-k value");

            #[test]
            fn succeeds_for_rsa() {
                let tag_value = "rsa";
                assert_eq!(SignatureAlgorithm::try_from(tag_value), Ok(SignatureAlgorithm::Rsa))
            }

            #[test]
            fn fails_for_ecdsa() {
                let tag_value = "ecdsa";
                assert_eq!(
                    SignatureAlgorithm::try_from(tag_value),
                    Err("Invalid header sig-a-tag-k value")
                );
            }
        }
    }
}
