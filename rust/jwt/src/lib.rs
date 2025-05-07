use derive_builder::Builder;
pub use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, decode_header,
    encode, errors::Error, get_current_timestamp,
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
    pub exp: u64,
    pub sub: String,
    #[builder(setter(into), default)]
    pub environment: Option<Environment>,
}

#[allow(clippy::unwrap_used)]
pub mod test_helpers {
    use super::*;

    pub struct TokenArgs<'a> {
        pub secret: &'a str,
        pub invalid_after: i64,
        pub subject: &'a str,
        pub host: Option<&'a str>,
        pub port: Option<u16>,
        pub environment: Option<Environment>,
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub fn token(args: &TokenArgs) -> String {
        let key = EncodingKey::from_secret(args.secret.as_bytes());
        let ts = get_current_timestamp() as i64 + args.invalid_after;
        let mut claims_builder = ClaimsBuilder::default()
            .exp(ts as u64)
            .sub(args.subject.to_string())
            .environment(args.environment);
        if let Some(host) = args.host {
            claims_builder = claims_builder
                .host(host.to_string())
                .port(args.port.unwrap_or(DEFAULT_WEB_PROOF_PORT))
        }

        let claims = claims_builder.build().unwrap();

        encode(&Header::default(), &claims, &key).unwrap()
    }

    pub const JWT_SECRET: &str = "deadbeef";
    pub const DEFAULT_WEB_PROOF_PORT: u16 = 443;
}

#[cfg(test)]
mod tests {
    use test_helpers::{JWT_SECRET, TokenArgs, token};

    use super::*;

    fn decoding_key() -> DecodingKey {
        DecodingKey::from_secret(JWT_SECRET.as_bytes())
    }

    #[test]
    fn decodes_without_web_proof_claims() {
        let jwt = token(&TokenArgs {
            secret: JWT_SECRET,
            host: None,
            port: None,
            invalid_after: 0,
            subject: "1234",
            environment: None,
        });
        let claims: TokenData<Claims> =
            decode(&jwt, &decoding_key(), &Validation::default()).unwrap();
        assert!(claims.claims.host.is_none());
        assert!(claims.claims.port.is_none());
    }

    #[test]
    fn decodes_with_web_proof_claims() {
        let jwt = token(&TokenArgs {
            secret: JWT_SECRET,
            host: Some("xyz.xyz"),
            port: None,
            invalid_after: 0,
            subject: "1234",
            environment: None,
        });
        let claims: TokenData<Claims> =
            decode(&jwt, &decoding_key(), &Validation::default()).unwrap();
        assert_eq!(claims.claims.host, Some("xyz.xyz".to_string()));
        assert_eq!(claims.claims.port, Some(443));
    }

    #[test]
    fn decodes_as_test() {
        let jwt = token(&TokenArgs {
            secret: JWT_SECRET,
            host: None,
            port: None,
            invalid_after: 0,
            subject: "1234",
            environment: Some(Environment::Test),
        });
        let token_data: TokenData<Claims> =
            decode(&jwt, &decoding_key(), &Validation::default()).unwrap();
        assert_eq!(token_data.claims.environment, Some(Environment::Test));
    }

    #[test]
    fn decodes_as_production() {
        let jwt = token(&TokenArgs {
            secret: JWT_SECRET,
            host: None,
            port: None,
            invalid_after: 0,
            subject: "1234",
            environment: Some(Environment::Production),
        });
        let token_data: TokenData<Claims> =
            decode(&jwt, &decoding_key(), &Validation::default()).unwrap();
        assert_eq!(token_data.claims.environment, Some(Environment::Production));
    }
}
