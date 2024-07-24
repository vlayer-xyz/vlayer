use crate::types::WebProof;

fn _verify_proof(_web_proof: WebProof) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use tlsn_core::proof::{SessionProof, TlsProof};

    use crate::fixtures::{notary_pub_key_example, tls_proof_example};
    use crate::types::WebProof;

    use super::_verify_proof;

    #[test]
    fn fail_verification() {
        let tls_proof = tls_proof_example();

        let wrong_tls_proof = TlsProof {
            session: SessionProof {
                signature: None,
                ..tls_proof.session
            },
            ..tls_proof
        };

        let proof = WebProof {
            tls_proof: wrong_tls_proof,
            notary_pub_key: notary_pub_key_example(),
        };
        assert_eq!(_verify_proof(proof), false)
    }

    #[test]
    fn success_verification() {
        let proof = WebProof {
            tls_proof: tls_proof_example(),
            notary_pub_key: notary_pub_key_example(),
        };
        assert_eq!(_verify_proof(proof), true)
    }
}
