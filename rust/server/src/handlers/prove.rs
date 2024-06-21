use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

use crate::{error::AppError, json::Json};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProveArgs {
    from: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProveResult {
    pub result: String,
}

pub(crate) async fn prove(Json(params): Json<ProveArgs>) -> Result<Json<ProveResult>, AppError> {
    let from: Address = params
        .from
        .parse()
        .map_err(|err| AppError::InvalidAddress {
            field: "from".to_string(),
            error: err,
        })?;
    Ok(Json(ProveResult {
        result: format!("I am {from}!"),
    }))
}

#[cfg(test)]
mod test {
    use alloy_primitives::hex::FromHexError;

    use crate::{error::AppError, json::Json};

    use super::{prove, ProveArgs};

    #[tokio::test]
    async fn success() -> anyhow::Result<()> {
        let actual = prove(Json(ProveArgs {
            from: "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".to_string(),
        }))
        .await?
        .0;

        assert_eq!(
            actual.result,
            "I am 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f!".to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn invalid_from_address() -> anyhow::Result<()> {
        let actual_err = prove(Json(ProveArgs {
            from: "0x".to_string(),
        }))
        .await
        .unwrap_err();

        assert_eq!(
            actual_err,
            AppError::InvalidAddress {
                field: "from".to_string(),
                error: FromHexError::InvalidStringLength,
            }
        );

        Ok(())
    }
}
