pub mod axum;
pub mod cli;

pub use jwt::{
    Algorithm, Claims, ClaimsBuilder, ClaimsBuilderError, DecodingKey, EncodingKey, Environment,
    Header, encode, get_current_timestamp,
};

pub mod test_helpers {
    use cli::Config;
    pub use jwt::test_helpers::{JWT_SECRET, TokenArgs, token};

    use super::*;

    pub fn default_config() -> Config {
        Config::new(DecodingKey::from_secret(JWT_SECRET.as_bytes()), Default::default())
    }
}
