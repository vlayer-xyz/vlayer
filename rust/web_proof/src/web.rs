use alloy_dyn_abi::DynSolValue;

#[derive(Debug, Clone)]
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
            self.notary_pub_key.into(),
        ]);
        data.abi_encode()
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::hex;

    use super::*;
    use crate::fixtures::NOTARY_PUB_KEY_PEM_EXAMPLE;

    #[test]
    fn test_abi_encoding() {
        // generated with `cast abi-encode "test(string[4])" "[https://api.x.com/1.1/account/settings.json,api.x.com,body,'-----BEGIN PUBLIC KEY-----
        // MDYwEAYHKoZIzj0CAQYFK4EEAAoDIgADe0jxnBObaIj7Xjg6TXLCM1GG/VhY5650
        // OrS/jgcbBuc=
        // -----END PUBLIC KEY-----']"`
        let expected_encoding: Vec<u8> = hex!("0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000000000160000000000000000000000000000000000000000000000000000000000000002b68747470733a2f2f6170692e782e636f6d2f312e312f6163636f756e742f73657474696e67732e6a736f6e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000096170692e782e636f6d00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004626f64790000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000812d2d2d2d2d424547494e205055424c4943204b45592d2d2d2d2d0a4d445977454159484b6f5a497a6a3043415159464b34454541416f444967414465306a786e424f6261496a37586a673654584c434d3147472f566859353635300a4f72532f6a6763624275633d0a2d2d2d2d2d454e44205055424c4943204b45592d2d2d2d2d00000000000000000000000000000000000000000000000000000000000000").to_vec();

        let web = Web {
            url: "https://api.x.com/1.1/account/settings.json".to_string(),
            server_name: "api.x.com".to_string(),
            body: "body".to_string(),
            notary_pub_key: NOTARY_PUB_KEY_PEM_EXAMPLE.to_string(),
        };

        assert_eq!(expected_encoding, web.abi_encode());
    }
}
