use serde::{Deserialize, Serialize};
use vlayer_engine::guest::Call as EngineCall;

use crate::{error::AppError, utils::parse_address_field};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Call {
    caller: String,
    to: String,
}

#[cfg(test)]
impl Call {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            caller: from.to_string(),
            to: to.to_string(),
        }
    }
}

impl TryFrom<Call> for EngineCall {
    type Error = AppError;

    fn try_from(value: Call) -> Result<Self, Self::Error> {
        Ok(Self {
            caller: parse_address_field("caller", value.caller)?,
            to: parse_address_field("to", value.to)?,
            data: Vec::new(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CallResult {
    pub result: String,
}

pub(crate) async fn call(params: Call) -> Result<CallResult, AppError> {
    let params: EngineCall = params.try_into()?;

    Ok(CallResult {
        result: format!("Call: caller {} to {}!", params.caller, params.to),
    })
}

#[cfg(test)]
mod test {
    use crate::handlers::v_call::Call;

    use super::call;

    const FROM: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";
    const INVALID_ADDRESS: &str = "0x";

    #[tokio::test]
    async fn success() -> anyhow::Result<()> {
        let actual = call(Call {
            caller: FROM.to_string(),
            to: TO.to_string(),
        })
        .await?;

        assert_eq!(
            actual.result,
            format!("Call: caller {FROM} to {TO}!").to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_from_address() -> anyhow::Result<()> {
        let actual_err = call(Call {
            caller: INVALID_ADDRESS.to_string(),
            to: TO.to_string(),
        })
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
        let actual_err = call(Call {
            caller: FROM.to_string(),
            to: INVALID_ADDRESS.to_string(),
        })
        .await
        .unwrap_err();

        assert_eq!(
            actual_err.to_string(),
            "Invalid field `to`: Invalid string length `0x`"
        );

        Ok(())
    }
}
