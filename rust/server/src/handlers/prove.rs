use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, FieldValidationError},
    json::Json,
};
use alloy_primitives::hex::FromHexError as AlloyFromHexError;
use hex::FromHexError;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProveArgsRpc {
    from: String,
    to: String,
}

pub struct ProveArgs {
    from: Address,
    to: Address,
}

impl TryFrom<ProveArgsRpc> for ProveArgs {
    type Error = AppError;

    fn try_from(value: ProveArgsRpc) -> Result<Self, Self::Error> {
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
        .map_err(|err| alloy_hex_error_to_standard_hex_error(err))
        .map_err(|err| {
            AppError::FieldValidationError(
                field_name.to_string(),
                FieldValidationError::InvalidHex(address, err),
            )
        })
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProveResult {
    pub result: String,
}

pub(crate) async fn prove(Json(params): Json<ProveArgsRpc>) -> Result<Json<ProveResult>, AppError> {
    let params: ProveArgs = params.try_into()?;

    Ok(Json(ProveResult {
        result: format!("Call: from {} to {}!", params.from, params.to),
    }))
}

#[cfg(test)]
mod test {
    use crate::{handlers::prove::ProveArgsRpc, json::Json};

    use super::prove;

    const FROM: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";
    const INVALID_ADDRESS: &str = "0x";

    #[tokio::test]
    async fn success() -> anyhow::Result<()> {
        let actual = prove(Json(ProveArgsRpc {
            from: FROM.to_string(),
            to: TO.to_string(),
        }))
        .await?
        .0;

        assert_eq!(
            actual.result,
            format!("Call: from {FROM} to {TO}!").to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_from_address() -> anyhow::Result<()> {
        let actual_err = prove(Json(ProveArgsRpc {
            from: INVALID_ADDRESS.to_string(),
            to: TO.to_string(),
        }))
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
        let actual_err = prove(Json(ProveArgsRpc {
            from: FROM.to_string(),
            to: INVALID_ADDRESS.to_string(),
        }))
        .await
        .unwrap_err();

        assert_eq!(
            actual_err.to_string(),
            "Invalid field `to`: Invalid string length `0x`"
        );

        Ok(())
    }
}
