use std::convert::Into;

use alloy_primitives::Bytes;
use alloy_sol_types::{SolCall, sol};
use web_proof::verifier::verify_and_parse;

use crate::helpers::{Result, map_to_fatal};

sol! {
    contract WebProof {
        #[derive(Debug)]
        struct Proof {
            string web_proof_json;
        }

        #[derive(Debug)]
        enum UrlTestMode {
            Full,
            Prefix
        }

        #[derive(Debug)]
        enum BodyRedactionMode {
            Disabled,
            Enabled_UNSAFE
        }

        function verify(Proof web_proof, UrlTestMode url_test_mode, BodyRedactionMode body_redaction_mode);
    }
}

pub(super) fn verify(input: &Bytes) -> Result<Bytes> {
    let WebProof::verifyCall {
        web_proof: WebProof::Proof { web_proof_json },
        ..
    } = WebProof::verifyCall::abi_decode_raw(input, true).map_err(map_to_fatal)?;
    let web_proof = serde_json::from_str(&web_proof_json).map_err(map_to_fatal)?;
    verify_and_parse(web_proof)
        .map(|x| x.abi_encode().into())
        .map_err(map_to_fatal)
}
