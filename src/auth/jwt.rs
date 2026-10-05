use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub roles: String,
    pub iat: i64,
}

#[derive(Clone)]
pub struct JwtService {
    decording_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            decording_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    fn verify_token(&self, token_str: &String) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        let token_data = decode(token_str, &self.decording_key, &validation)?;
        Ok(token_data.claims)
    }
}
