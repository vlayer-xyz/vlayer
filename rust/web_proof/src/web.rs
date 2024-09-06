use alloy_dyn_abi::DynSolValue;

pub struct Web {
    pub url: String,
    pub server_name: String,
    pub body: String,
    pub notary_pub_key: String,
}

impl Web {
    pub fn abi_encode(self) -> Vec<u8> {
        let data = DynSolValue::FixedArray(vec![
            self.url.into(),
            self.server_name.into(),
            self.body.into(),
        ]);
        data.abi_encode()
    }
}

#[cfg(test)]
mod tests {
    use crate::fixtures::NOTARY_PUB_KEY_PEM_EXAMPLE;

    use super::*;

    use alloy_primitives::hex;

    #[test]
    fn test_abi_encoding() {
        // generated with `cast abi-encode "test(string[3])" "[https://api.x.com/1.1/account/settings.json,api.x.com,body]"`
        let expected_encoding: Vec<u8> = hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000002b68747470733a2f2f6170692e782e636f6d2f312e312f6163636f756e742f73657474696e67732e6a736f6e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000096170692e782e636f6d00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004626f647900000000000000000000000000000000000000000000000000000000").to_vec();

        let web = Web {
            url: "https://api.x.com/1.1/account/settings.json".to_string(),
            server_name: "api.x.com".to_string(),
            body: "body".to_string(),
            notary_pub_key: NOTARY_PUB_KEY_PEM_EXAMPLE.to_string(),
        };

        assert_eq!(expected_encoding, web.abi_encode());
    }
}
