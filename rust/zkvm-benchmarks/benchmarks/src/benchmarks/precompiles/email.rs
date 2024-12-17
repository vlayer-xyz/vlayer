use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use call_precompiles::verify_and_parse_email::verify_and_parse_run as verify;
use email_proof::UnverifiedEmail;

use crate::{with_fixture, Benchmark};

const DNS_RECORD: &str = "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoDLLSKLb3eyflXzeHwBz8qqg9mfpmMY+f1tp+HpwlEOeN5iHO0s4sCd2QbG2i/DJRzryritRnjnc4i2NJ/IJfU8XZdjthotcFUY6rrlFld20a13q8RYBBsETSJhYnBu+DMdIF9q3YxDtXRFNpFCpI1uIeA/x+4qQJm3KTZQWdqi/BVnbsBA6ZryQCOOJC3Ae0oodvz80yfEJUAi9hAGZWqRn+Mprlyu749uQ91pTOYCDCbAn+cqhw8/mY5WMXFqrw9AdfWrk+MwXHPVDWBs8/Hm8xkWxHOqYs9W51oZ/Je3WWeeggyYCZI9V+Czv7eF8BD/yF9UxU/3ZWZPM8EWKKQIDAQAB";
const SMALL_EMAIL: &str = include_str!(concat!("../../../assets/email.eml"));

fn fixture() -> Bytes {
    let email: UnverifiedEmail = UnverifiedEmail {
        email: SMALL_EMAIL.to_string(),
        dnsRecords: vec![DNS_RECORD.to_string()],
    };

    email.abi_encode().into()
}

fn test_email_verification(calldata: Bytes) {
    let _ = verify(&calldata, 100_000_000).expect("Verification failed");
}

pub fn benchmarks() -> Vec<Benchmark> {
    vec![Benchmark::new(
        "email_validation",
        with_fixture!(fixture(), test_email_verification),
        44_578_000,
    )]
}
