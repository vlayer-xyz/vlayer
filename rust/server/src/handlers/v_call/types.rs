use crate::error::AppError;
use crate::utils::{parse_address_field, parse_hex_field};
use alloy_chains::Chain;
use alloy_primitives::{BlockNumber, ChainId};
use host::Call as HostCall;
use p256::PublicKey;
use pkcs8::DecodePublicKey;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use tlsn_core::proof::TlsProof;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Call {
    caller: String,
    to: String,
    data: String,
}

impl TryFrom<Call> for HostCall {
    type Error = AppError;

    fn try_from(value: Call) -> Result<Self, Self::Error> {
        Ok(Self {
            caller: parse_address_field("caller", value.caller)?,
            to: parse_address_field("to", value.to)?,
            data: parse_hex_field("data", value.data)?,
        })
    }
}

fn mainnet_chain_id() -> ChainId {
    Chain::mainnet().id()
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallContext {
    pub block_no: BlockNumber,
    #[serde(default = "mainnet_chain_id")]
    pub chain_id: ChainId,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Augmentors {
    pub web_proof: WebProof,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebProof {
    #[serde(deserialize_with = "deserialize_public_key_from_pem_string")]
    pub notary_pub_key: PublicKey,
    pub tls_proof: TlsProof,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CallResult {
    pub result: String,
}

fn deserialize_public_key_from_pem_string<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    let key_pem = String::deserialize(deserializer)?;
    PublicKey::from_public_key_pem(&key_pem).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod test {
    use super::Call;
    use crate::error::AppError;
    use host::Call as HostCall;

    const FROM: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";
    const DATA: &str = "0x0000";
    const INVALID_ADDRESS: &str = "0x";

    #[tokio::test]
    async fn invalid_from_address() -> anyhow::Result<()> {
        let call = Call {
            caller: INVALID_ADDRESS.to_string(),
            to: TO.to_string(),
            data: DATA.to_string(),
        };
        let actual_result: Result<HostCall, AppError> = call.try_into();

        assert_eq!(
            actual_result.unwrap_err().to_string(),
            "Invalid field `caller`: Invalid string length `0x`"
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_to_address() -> anyhow::Result<()> {
        let call = Call {
            caller: FROM.to_string(),
            to: INVALID_ADDRESS.to_string(),
            data: DATA.to_string(),
        };
        let actual_result: Result<HostCall, AppError> = call.try_into();

        assert_eq!(
            actual_result.unwrap_err().to_string(),
            "Invalid field `to`: Invalid string length `0x`"
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_data() -> anyhow::Result<()> {
        const INVALID_DATA: &str = "xx";
        let call = Call {
            caller: FROM.to_string(),
            to: TO.to_string(),
            data: INVALID_DATA.to_string(),
        };
        let actual_result: Result<HostCall, AppError> = call.try_into();

        assert_eq!(
            actual_result.unwrap_err().to_string(),
            "Invalid field `data`: Invalid hex prefix `xx`"
        );

        Ok(())
    }
}
