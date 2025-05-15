use derive_builder::Builder;
pub use jwt_simple::{
    JWTError,
    algorithms::{MACLike, RS256KeyPair},
    claims::Claims as RawClaims,
    prelude::Duration,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    Test,
    Production,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[builder(pattern = "owned")]
pub struct Claims {
    #[builder(setter(into, strip_option), default)]
    pub host: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub port: Option<u16>,
    #[builder(setter(into), default)]
    pub environment: Option<Environment>,
}

#[allow(clippy::unwrap_used)]
pub mod test_helpers {
    use std::cell::LazyCell;

    use jwt_simple::algorithms::HS256Key;

    use super::*;

    pub struct TokenArgs<'a> {
        pub symmetric_key: &'a HS256Key,
        pub invalid_after: Duration,
        pub subject: &'a str,
        pub host: Option<&'a str>,
        pub port: Option<u16>,
        pub environment: Option<Environment>,
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub fn token(args: &TokenArgs) -> String {
        let mut claims_builder = ClaimsBuilder::default().environment(args.environment);

        if let Some(host) = args.host {
            claims_builder = claims_builder
                .host(host.to_string())
                .port(args.port.unwrap_or(DEFAULT_WEB_PROOF_PORT))
        }

        let claims = claims_builder.build().unwrap();
        let all_claims =
            RawClaims::with_custom_claims(claims, args.invalid_after).with_subject(args.subject);

        args.symmetric_key.authenticate(all_claims).unwrap()
    }

    pub const JWT_KEY: LazyCell<Box<[u8]>> =
        LazyCell::new(|| HS256Key::generate().to_bytes().into_boxed_slice());
    pub const DEFAULT_WEB_PROOF_PORT: u16 = 443;
}

#[cfg(test)]
mod tests {
    use jwt_simple::prelude::HS256Key;
    use test_helpers::{JWT_KEY, TokenArgs, token};

    use super::*;

    fn symmetric_key() -> HS256Key {
        HS256Key::from_bytes(&JWT_KEY)
    }

    #[test]
    fn decodes_without_web_proof_claims() {
        let key = symmetric_key();
        let jwt = token(&TokenArgs {
            symmetric_key: &key,
            host: None,
            port: None,
            invalid_after: Duration::from_secs(10),
            subject: "1234",
            environment: None,
        });
        let claims = key.verify_token::<Claims>(&jwt, None).unwrap();
        assert!(claims.custom.host.is_none());
        assert!(claims.custom.port.is_none());
    }

    #[test]
    fn decodes_with_web_proof_claims() {
        let key = symmetric_key();
        let jwt = token(&TokenArgs {
            symmetric_key: &key,
            host: Some("xyz.xyz"),
            port: None,
            invalid_after: Duration::from_secs(10),
            subject: "1234",
            environment: None,
        });
        let claims = key.verify_token::<Claims>(&jwt, None).unwrap();
        assert_eq!(claims.custom.host, Some("xyz.xyz".to_string()));
        assert_eq!(claims.custom.port, Some(443));
    }

    #[test]
    fn decodes_as_test() {
        let key = symmetric_key();
        let jwt = token(&TokenArgs {
            symmetric_key: &key,
            host: None,
            port: None,
            invalid_after: Duration::from_secs(10),
            subject: "1234",
            environment: Some(Environment::Test),
        });
        let claims = key.verify_token::<Claims>(&jwt, None).unwrap();
        assert_eq!(claims.custom.environment, Some(Environment::Test));
    }

    #[test]
    fn decodes_as_production() {
        let key = symmetric_key();
        let jwt = token(&TokenArgs {
            symmetric_key: &key,
            host: None,
            port: None,
            invalid_after: Duration::from_secs(10),
            subject: "1234",
            environment: Some(Environment::Production),
        });
        let claims = key.verify_token::<Claims>(&jwt, None).unwrap();
        assert_eq!(claims.custom.environment, Some(Environment::Production));
    }
}
