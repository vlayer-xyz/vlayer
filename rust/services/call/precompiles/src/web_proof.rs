use std::convert::Into;

use alloy_primitives::Bytes;
use alloy_sol_types::{SolCall, sol};
use web_proof::{
    verifier::verify_and_parse,
    web_proof::{BodyRedactionMode, Config, UrlTestMode},
};

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

impl From<WebProof::BodyRedactionMode> for BodyRedactionMode {
    fn from(mode: WebProof::BodyRedactionMode) -> Self {
        match mode {
            WebProof::BodyRedactionMode::Disabled => BodyRedactionMode::Disabled,
            WebProof::BodyRedactionMode::Enabled_UNSAFE => BodyRedactionMode::EnabledUnsafe,
            WebProof::BodyRedactionMode::__Invalid => {
                unreachable!("Invalid BodyRedactionMode")
            }
        }
    }
}

impl From<WebProof::UrlTestMode> for UrlTestMode {
    fn from(mode: WebProof::UrlTestMode) -> Self {
        match mode {
            WebProof::UrlTestMode::Full => UrlTestMode::Full,
            WebProof::UrlTestMode::Prefix => UrlTestMode::Prefix,
            WebProof::UrlTestMode::__Invalid => {
                unreachable!("Invalid UrlTestMode")
            }
        }
    }
}

pub(super) fn verify(input: &Bytes) -> Result<Bytes> {
    let WebProof::verifyCall {
        web_proof: WebProof::Proof { web_proof_json },
        url_test_mode,
        body_redaction_mode,
    } = WebProof::verifyCall::abi_decode_raw(input, true).map_err(map_to_fatal)?;
    let web_proof = serde_json::from_str(&web_proof_json).map_err(map_to_fatal)?;
    let config = Config::new(body_redaction_mode, url_test_mode);
    verify_and_parse(web_proof, config)
        .map(|x| x.abi_encode().into())
        .map_err(map_to_fatal)
}
