use serde::Serialize;

pub(crate) trait ToPayload {
    fn to_payload(&self) -> Vec<u8>;
}

impl<T: Serialize> ToPayload for T {
    #[allow(clippy::expect_used)]
    fn to_payload(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let formattter = olpc_cjson::CanonicalFormatter::new();
        let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formattter);
        self.serialize(&mut serializer)
            .expect("Failed to serialize signable struct");

        buf
    }
}
