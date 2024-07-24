use core::str;
use serde_json::Value;

pub const NOTARY_PUB_KEY: &str = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

pub fn tls_proof() -> Value {
    serde_json::from_str(str::from_utf8(include_bytes!("../testdata/tls_proof.json")).unwrap())
        .unwrap()
}
