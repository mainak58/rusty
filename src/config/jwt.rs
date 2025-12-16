use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

use crate::dto::auth::user_dto::UserDto;

#[derive(Debug, Serialize)]
struct Claims {
  sub: String,
  exp: usize,
  role: String,
}

pub fn create_jwt(user: &UserDto, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
  let expire = Utc::now()
    .checked_add_signed(Duration::hours(48))
    .expect("valid time")
    .timestamp() as usize;

  let claims = Claims {
    sub: user.id.to_string(),
    exp: expire,
    role: format!("{:?}", user.role),
  };

  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  )
}
