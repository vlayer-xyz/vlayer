use core::str;
use serde_json::Value;

pub fn tls_proof() -> Value {
    serde_json::from_str(str::from_utf8(include_bytes!("../testdata/tls_proof.json")).unwrap())
        .unwrap()
}
