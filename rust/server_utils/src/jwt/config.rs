use derive_more::Debug;
use derive_new::new;
use jwt::{Claim as JwtClaim, DecodingKey, JwtAlgorithm};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[error("JWT validation error: {0}")]
pub struct ValidationError(String);

type Result<T> = std::result::Result<T, ValidationError>;

#[derive(new, Clone, Debug)]
pub struct Config {
    #[debug(skip)]
    pub public_key: DecodingKey,
    pub algorithm: JwtAlgorithm,
    pub claims: Vec<JwtClaim>,
}

impl Config {
    pub fn validate(&self, claims: &Value) -> Result<()> {
        Self::validate_claims(&self.claims, claims)
    }

    fn validate_claims(expected: &[JwtClaim], claims: &Value) -> Result<()> {
        expected
            .iter()
            .try_for_each(|expected| Self::validate_claim(expected, claims))
    }

    fn validate_claim(expected: &JwtClaim, given: &Value) -> Result<()> {
        let pointer = format!("/{}", expected.name.replace(".", "/"));
        let field = given
            .pointer(&pointer)
            .ok_or(ValidationError(format!("missing claim '{}'", expected.name)))?;

        let field_typed = field.as_str().ok_or(ValidationError(format!(
            "unexpected type for claim '{}': only strings are supported for claim values",
            expected.name,
        )))?;
        if !expected.values.is_empty() {
            expected.values.iter().any(|exp| exp == field_typed).then_some(()).ok_or_else(|| {
                        let expected_values = expected.values.iter().map(|x| format!("'{x}'")).collect::<Vec<String>>().join(", ");
                        ValidationError(format!(
                            "unexpected value for claim '{}': expected one of [ {expected_values} ], received '{field_typed}'", expected.name
                        ))
                    })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn validates_presence() {
        let expected = JwtClaim {
            name: "sub".to_string(),
            ..Default::default()
        };
        let given = json!({
            "exp": 12345,
            "sub": "test",
        });
        Config::validate_claim(&expected, &given).unwrap();
    }

    #[test]
    fn validates_expected_value() {
        let expected = JwtClaim {
            name: "custom.host".to_string(),
            values: vec!["tlsn.com".to_string(), "api.tlsn.com".to_string()],
        };
        let given = json!({
            "exp": 12345,
            "custom": {
                "host": "api.tlsn.com",
            },
        });
        Config::validate_claim(&expected, &given).unwrap();
    }

    #[test]
    fn validates_with_unknown_claims() {
        let given = json!({
            "exp": 12345,
            "sub": "test",
            "what": "is_this",
        });
        Config::validate_claims(&[], &given).unwrap();
    }

    #[test]
    fn fails_if_claim_missing() {
        let expected = JwtClaim {
            name: "sub".to_string(),
            ..Default::default()
        };
        let given = json!({
            "exp": 12345,
            "host": "localhost",
        });
        assert_eq!(
            Config::validate_claim(&expected, &given),
            Err(ValidationError("missing claim 'sub'".to_string()))
        )
    }

    #[test]
    fn fails_if_claim_has_unknown_value() {
        let expected = JwtClaim {
            name: "sub".to_string(),
            values: vec!["tlsn_prod".to_string(), "tlsn_test".to_string()],
        };
        let given = json!({
            "sub": "tlsn",
        });
        assert_eq!(
                Config::validate_claim(&expected, &given),
                Err(ValidationError("unexpected value for claim 'sub': expected one of [ 'tlsn_prod', 'tlsn_test' ], received 'tlsn'".to_string()))
            )
    }

    #[test]
    fn fails_if_claim_has_invalid_value_type() {
        let expected = JwtClaim {
            name: "sub".to_string(),
            ..Default::default()
        };
        let given = json!({
            "sub": { "name": "john" }
        });
        assert_eq!(
            Config::validate_claim(&expected, &given),
            Err(ValidationError(
                "unexpected type for claim 'sub': only strings are supported for claim values"
                    .to_string()
            ))
        )
    }
}
