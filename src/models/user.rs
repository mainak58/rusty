// pub enum UserRole {
//   Superadmin,
//   Admin,
//   Seller,
//   User,
// }

use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
  Superadmin,
  Admin,
  Seller,
  User,
}
