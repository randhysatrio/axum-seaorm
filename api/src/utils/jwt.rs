use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use std::env;

use crate::errors::APIResult;

lazy_static! {
    static ref JWT_KEY: String = env::var("JWT_KEY").expect("JWT_KEY must be set in .env");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub user_id: i32,
}

impl Claims {
    pub fn new(user_id: i32) -> Self {
        Self {
            user_id,
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
        }
    }
}

pub fn generate_token(user_id: i32) -> APIResult<String> {
    let token = jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(user_id),
        &EncodingKey::from_secret(JWT_KEY.as_bytes()),
    )?;

    Ok(token)
}

pub fn verify_token(token: &str) -> APIResult<TokenData<Claims>> {
    let token = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_KEY.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token)
}
