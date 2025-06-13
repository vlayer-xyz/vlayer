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
        validate_claims(&self.claims, claims)
    }
}

fn validate_claims(expected: &[JwtClaim], claims: &Value) -> Result<()> {
    expected
        .iter()
        .try_for_each(|expected| validate_claim(expected, claims))
}

fn validate_claim(expected: &JwtClaim, given: &Value) -> Result<()> {
    let name = &expected.name;
    let field = extract_claim_by_name(name, given)?;
    if let Some(typed) = field.as_str() {
        validate_string_claim(expected, typed)
    } else {
        Err(unexpected_type(name))
    }
}

fn extract_claim_by_name<'a>(name: &'a str, given: &'a Value) -> Result<&'a Value> {
    let pointer = format!("/{}", name.replace(".", "/"));
    given
        .pointer(&pointer)
        .ok_or(ValidationError(format!("missing claim '{name}'")))
}

fn validate_string_claim(expected: &JwtClaim, given: &str) -> Result<()> {
    if !expected.values.is_empty() {
        expected
            .values
            .iter()
            .any(|exp| exp == given)
            .then_some(())
            .ok_or_else(|| unexpected_value(expected, &given))?;
    }
    Ok(())
}

fn unexpected_type(name: &str) -> ValidationError {
    ValidationError(format!(
        "unexpected type for claim '{name}': only strings are supported for claim values",
    ))
}

fn unexpected_value(expected: &JwtClaim, given: &impl ToString) -> ValidationError {
    let name = &expected.name;
    let given = given.to_string();
    let expected_values = expected
        .values
        .iter()
        .map(|x| format!("'{x}'"))
        .collect::<Vec<String>>()
        .join(", ");
    ValidationError(format!(
        "unexpected value for claim '{name}': expected one of [ {expected_values} ], received '{given}'"
    ))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    mod validate_claims {
        use super::*;

        #[test]
        fn validates_with_unknown_claims() {
            let given = json!({
                "exp": 12345,
                "sub": "test",
                "what": "is_this",
            });
            validate_claims(&[], &given).unwrap();
        }
    }

    mod validate_claim {
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
            validate_claim(&expected, &given).unwrap();
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
            validate_claim(&expected, &given).unwrap();
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
                validate_claim(&expected, &given),
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
                validate_claim(&expected, &given),
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
                validate_claim(&expected, &given),
                Err(ValidationError(
                    "unexpected type for claim 'sub': only strings are supported for claim values"
                        .to_string()
                ))
            )
        }
    }
}
