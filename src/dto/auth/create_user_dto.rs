use serde::Deserialize;

use crate::models::user::UserRole;

#[derive(Deserialize)]
pub struct CreateUserDto {
  pub name: String,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub password: String,
  pub role: Option<UserRole>,
}
