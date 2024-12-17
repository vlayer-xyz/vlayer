use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use call_precompiles::verify_and_parse_email::verify_and_parse_run as verify;
use email_proof::UnverifiedEmail;

use crate::{with_fixture, Benchmark};

const DNS_RECORD: &str = "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3gWcOhCm99qzN+h7/2+LeP3CLsJkQQ4EP/2mrceXle5pKq8uZmBl1U4d2Vxn4w+pWFANDLmcHolLboESLFqEL5N6ae7u9b236dW4zn9AFkXAGenTzQEeif9VUFtLAZ0Qh2eV7OQgz/vPj5IaNqJ7h9hpM9gO031fe4v+J0DLCE8Rgo7hXbNgJavctc0983DaCDQaznHZ44LZ6TtZv9TBs+QFvsy4+UCTfsuOtHzoEqOOuXsVXZKLP6B882XbEnBpXEF8QzV4J26HiAJFUbO3mAqZL2UeKC0hhzoIZqZXNG0BfuzOF0VLpDa18GYMUiu+LhEJPJO9D8zhzvQIHNrpGwIDAQAB";
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
