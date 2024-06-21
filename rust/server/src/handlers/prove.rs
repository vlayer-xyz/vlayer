use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, FieldValidationError},
    json::Json,
};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProveArgs {
    from: String,
    to: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProveResult {
    pub result: String,
}

pub(crate) async fn prove(Json(params): Json<ProveArgs>) -> Result<Json<ProveResult>, AppError> {
    let from: Address = params
        .from
        .parse()
        .map_err(|err| AppError::FieldValidationError {
            field: "from".to_string(),
            error: FieldValidationError::InvalidHex(params.from, err),
        })?;

    let to: Address = params
        .to
        .parse()
        .map_err(|err| AppError::FieldValidationError {
            field: "to".to_string(),
            error: FieldValidationError::InvalidHex(params.to, err),
        })?;

    Ok(Json(ProveResult {
        result: format!("Call: from {from} to {to}!"),
    }))
}

#[cfg(test)]
mod test {
    use crate::json::Json;

    use super::{prove, ProveArgs};

    const FROM: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";
    const INVALID_ADDRESS: &str = "0x";

    #[tokio::test]
    async fn success() -> anyhow::Result<()> {
        let actual = prove(Json(ProveArgs {
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
        let actual_err = prove(Json(ProveArgs {
            from: INVALID_ADDRESS.to_string(),
            to: TO.to_string(),
        }))
        .await
        .unwrap_err();

        assert_eq!(
            actual_err.to_string(),
            "Invalid field `from`: invalid string length `0x`"
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_to_address() -> anyhow::Result<()> {
        let actual_err = prove(Json(ProveArgs {
            from: FROM.to_string(),
            to: INVALID_ADDRESS.to_string(),
        }))
        .await
        .unwrap_err();

        assert_eq!(
            actual_err.to_string(),
            "Invalid field `to`: invalid string length `0x`"
        );

        Ok(())
    }
}
