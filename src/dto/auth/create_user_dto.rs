use serde::{Deserialize, Serialize};

use crate::models::user::UserRole;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUserDto {
  pub name: String,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub password: String,
  pub role: Option<UserRole>,
}

#[derive(Serialize, Default)]
pub struct UserResponseDto {
  pub name: String,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub role: Option<String>,
}
