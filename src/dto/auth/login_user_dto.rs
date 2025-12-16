use serde::{Deserialize, Serialize};

use crate::models::user::UserRole;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginUserDto {
  pub email: Option<String>,
  pub phone: Option<String>,
  pub password: String,
  pub role: UserRole,
}

pub enum LoginIdentifier<'a> {
  Email(&'a str),
  Phone(&'a str),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginDto {
  pub email: Option<String>,
  pub password: String,
}
