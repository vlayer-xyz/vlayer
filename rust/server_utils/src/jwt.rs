pub mod axum;
pub mod cli;

use derive_new::new;
pub use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, encode, get_current_timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub host: String,
    pub port: u16,
    pub exp: u64,
    pub sub: String,
}

pub mod test_helpers {
    use cli::Config;

    use super::*;

    pub struct TokenArgs<'a> {
        pub secret: &'a str,
        pub host: &'a str,
        pub port: u16,
        pub invalid_after: i64,
        pub subject: &'a str,
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub fn token(args: &TokenArgs) -> String {
        let key = EncodingKey::from_secret(args.secret.as_bytes());
        let ts = get_current_timestamp() as i64 + args.invalid_after;
        let claims =
            Claims::new(args.host.to_string(), args.port, ts as u64, args.subject.to_string());
        encode(&Header::default(), &claims, &key).unwrap()
    }

    pub const JWT_SECRET: &str = "deadbeef";

    pub fn default_config() -> Config {
        Config::new(DecodingKey::from_secret(JWT_SECRET.as_bytes()), Default::default())
    }
}
