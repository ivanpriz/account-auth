use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Duration;
use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};

use crate::config::CONFIG;

#[derive(serde::Serialize)]
pub struct Claims {
    pub exp: usize,       // Expiry time of the token
    pub iat: usize,       // Issued at time of the token
    pub username: String, // Email associated with the token
}

pub fn encode_jwt(username: &str) -> Result<String, Error> {
    let now = chrono::Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims {
        iat,
        exp,
        username: username.to_string(),
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
    )
}

pub fn hash_pwd(raw_pwd: &str) -> String {
    hash(raw_pwd, DEFAULT_COST).unwrap()
}

pub fn verify_pwd(raw_pwd: &str, hashed_pwd: &str) -> bool {
    verify(raw_pwd, hashed_pwd).unwrap()
}
