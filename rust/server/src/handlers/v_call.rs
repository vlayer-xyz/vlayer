use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, FieldValidationError};
use alloy_primitives::hex::FromHexError as AlloyFromHexError;
use hex::FromHexError;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallArgsRpc {
    from: String,
    to: String,
}

#[cfg(test)]
impl CallArgsRpc {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}

pub struct CallArgs {
    from: Address,
    to: Address,
}

impl TryFrom<CallArgsRpc> for CallArgs {
    type Error = AppError;

    fn try_from(value: CallArgsRpc) -> Result<Self, Self::Error> {
        Ok(Self {
            from: parse_address_field("from", value.from)?,
            to: parse_address_field("to", value.to)?,
        })
    }
}

fn alloy_hex_error_to_standard_hex_error(err: AlloyFromHexError) -> FromHexError {
    match err {
        AlloyFromHexError::InvalidHexCharacter { c, index } => {
            FromHexError::InvalidHexCharacter { c, index }
        }
        AlloyFromHexError::InvalidStringLength => FromHexError::InvalidStringLength,
        AlloyFromHexError::OddLength => FromHexError::OddLength,
    }
}

fn parse_address_field(field_name: &str, address: String) -> Result<Address, AppError> {
    address
        .parse()
        .map_err(alloy_hex_error_to_standard_hex_error)
        .map_err(|err| {
            AppError::FieldValidationError(
                field_name.to_string(),
                FieldValidationError::InvalidHex(address, err),
            )
        })
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CallResult {
    pub result: String,
}

pub(crate) async fn call(params: CallArgsRpc) -> Result<CallResult, AppError> {
    let params: CallArgs = params.try_into()?;

    Ok(CallResult {
        result: format!("Call: from {} to {}!", params.from, params.to),
    })
}

#[cfg(test)]
mod test {
    use crate::handlers::v_call::CallArgsRpc;

    use super::call;

    const FROM: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";
    const INVALID_ADDRESS: &str = "0x";

    #[tokio::test]
    async fn success() -> anyhow::Result<()> {
        let actual = call(CallArgsRpc {
            from: FROM.to_string(),
            to: TO.to_string(),
        })
        .await?;

        assert_eq!(
            actual.result,
            format!("Call: from {FROM} to {TO}!").to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_from_address() -> anyhow::Result<()> {
        let actual_err = call(CallArgsRpc {
            from: INVALID_ADDRESS.to_string(),
            to: TO.to_string(),
        })
        .await
        .unwrap_err();

        assert_eq!(
            actual_err.to_string(),
            "Invalid field `from`: Invalid string length `0x`"
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_to_address() -> anyhow::Result<()> {
        let actual_err = call(CallArgsRpc {
            from: FROM.to_string(),
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
