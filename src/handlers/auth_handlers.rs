use axum::{
  extract::{Json, State},
  http::StatusCode,
};
use sqlx::PgPool;

use crate::{dto::auth::create_user_dto::CreateUserDto, services::auth_services};

pub async fn create_user(
  State(pool): State<PgPool>,
  Json(payload): Json<CreateUserDto>,
) -> (StatusCode, &'static str) {
  match auth_services::create_user(&pool, payload).await {
    Ok(_) => (StatusCode::CREATED, "User created"),
    Err(_) => (StatusCode::BAD_REQUEST, "Failed to create user"),
  }
}
