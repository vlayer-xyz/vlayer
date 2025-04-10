use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use call_precompiles::email_proof::verify;
use email_proof::{SolDnsRecord, SolVerificationData, UnverifiedEmail};

use crate::{Benchmark, with_fixture};

const DNS_RECORD: &str = "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoDLLSKLb3eyflXzeHwBz8qqg9mfpmMY+f1tp+HpwlEOeN5iHO0s4sCd2QbG2i/DJRzryritRnjnc4i2NJ/IJfU8XZdjthotcFUY6rrlFld20a13q8RYBBsETSJhYnBu+DMdIF9q3YxDtXRFNpFCpI1uIeA/x+4qQJm3KTZQWdqi/BVnbsBA6ZryQCOOJC3Ae0oodvz80yfEJUAi9hAGZWqRn+Mprlyu749uQ91pTOYCDCbAn+cqhw8/mY5WMXFqrw9AdfWrk+MwXHPVDWBs8/Hm8xkWxHOqYs9W51oZ/Je3WWeeggyYCZI9V+Czv7eF8BD/yF9UxU/3ZWZPM8EWKKQIDAQAB";
const SMALL_EMAIL: &str = include_str!(concat!("../../../assets/email.eml"));

fn fixture() -> Bytes {
    let email: UnverifiedEmail = UnverifiedEmail {
        email: SMALL_EMAIL.to_string(),
        dnsRecord: SolDnsRecord {
            name: "google._domainkey.vlayer.xyz".into(),
            recordType: 16,
            data: DNS_RECORD.into(),
            ttl: 0,
        },
        verificationData: SolVerificationData {
            validUntil: 0,
            signature: Default::default(),
            pubKey: Default::default(),
        },
    };

    email.abi_encode().into()
}

fn test_email_verification(calldata: Bytes) {
    let _ = verify(&calldata).expect("Verification failed");
}

pub fn benchmarks() -> Vec<Benchmark> {
    vec![Benchmark::new(
        "email_validation",
        with_fixture!(fixture(), test_email_verification),
        10_565_574,
    )]
}
