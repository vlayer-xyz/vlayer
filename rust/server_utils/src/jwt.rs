pub mod axum;
pub mod cli;
pub mod config;

pub use jwt::{
    Algorithm, Claim, Claims, ClaimsBuilder, ClaimsBuilderError, DecodingKey, EncodingKey,
    Environment, Header, encode, get_current_timestamp,
};

pub mod test_helpers {
    use config::Config;
    pub use jwt::test_helpers::{JWT_SECRET, TokenArgs, token};

    use super::*;

    pub fn default_config() -> Config {
        Config::new(
            DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            Default::default(),
            vec![
                Claim {
                    name: "sub".to_string(),
                    values: vec![],
                },
                Claim {
                    name: "environment".to_string(),
                    values: vec!["test".to_string()],
                },
            ],
        )
    }
}
