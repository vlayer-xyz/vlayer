use crate::{
    error::AppError,
    utils::{parse_address_field, parse_hex_field},
};
use alloy_chains::Chain;
use alloy_primitives::{BlockNumber, ChainId};
use host::Call as HostCall;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Call {
    caller: String,
    to: String,
    data: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallContext {
    block_no: BlockNumber,
    #[serde(default = "mainnet_chain_id")]
    chain_id: ChainId,
}

fn mainnet_chain_id() -> ChainId {
    Chain::mainnet().id()
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

#[derive(Deserialize, Serialize, Debug)]
pub struct CallResult {
    pub result: String,
}

pub(crate) async fn call(params: (Call, CallContext)) -> Result<CallResult, AppError> {
    let call: HostCall = params.0.try_into()?;
    let context = params.1;

    Ok(CallResult {
        result: format!(
            "Call: caller {} to {} with data {:?}. Context: block {} chain {}.",
            call.caller, call.to, call.data, context.block_no, context.chain_id
        ),
    })
}

#[cfg(test)]
mod test {
    use crate::handlers::v_call::{mainnet_chain_id, Call, CallContext};

    use super::call;

    const FROM: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";
    const DATA: &str = "0x0000";
    const INVALID_ADDRESS: &str = "0x";

    #[tokio::test]
    async fn success() -> anyhow::Result<()> {
        let actual = call((
            Call {
                caller: FROM.to_string(),
                to: TO.to_string(),
                data: DATA.to_string(),
            },
            CallContext {
                block_no: 0,
                chain_id: mainnet_chain_id(),
            },
        ))
        .await?;

        assert_eq!(
            actual.result,
            format!("Call: caller {FROM} to {TO} with data [0, 0]. Context: block 0 chain 1.")
                .to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_from_address() -> anyhow::Result<()> {
        let actual_err = call((
            Call {
                caller: INVALID_ADDRESS.to_string(),
                to: TO.to_string(),
                data: DATA.to_string(),
            },
            CallContext {
                block_no: 0,
                chain_id: mainnet_chain_id(),
            },
        ))
        .await
        .unwrap_err();

        assert_eq!(
            actual_err.to_string(),
            "Invalid field `caller`: Invalid string length `0x`"
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_to_address() -> anyhow::Result<()> {
        let actual_err = call((
            Call {
                caller: FROM.to_string(),
                to: INVALID_ADDRESS.to_string(),
                data: DATA.to_string(),
            },
            CallContext {
                block_no: 0,
                chain_id: mainnet_chain_id(),
            },
        ))
        .await
        .unwrap_err();

        assert_eq!(
            actual_err.to_string(),
            "Invalid field `to`: Invalid string length `0x`"
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_data() -> anyhow::Result<()> {
        const INVALID_DATA: &str = "xx";
        let actual_err = call((
            Call {
                caller: FROM.to_string(),
                to: TO.to_string(),
                data: INVALID_DATA.to_string(),
            },
            CallContext {
                block_no: 0,
                chain_id: mainnet_chain_id(),
            },
        ))
        .await
        .unwrap_err();

        assert_eq!(
            actual_err.to_string(),
            "Invalid field `data`: Invalid hex prefix `xx`"
        );

        Ok(())
    }
}
