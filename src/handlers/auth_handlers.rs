use crate::{
  dto::{
    auth::create_user_dto::{CreateUserDto, UserResponseDto},
    common::common_dto::ApiResponse,
  },
  services::auth_services,
};
use axum::{
  extract::{Json, State},
  http::StatusCode,
  response::IntoResponse,
};
use sqlx::PgPool;

pub async fn create_user(
  State(pool): State<PgPool>,
  Json(payload): Json<CreateUserDto>,
) -> impl IntoResponse {
  match auth_services::create_user(&pool, &payload).await {
    Ok(_) => {
      let response = UserResponseDto {
        name: payload.name,
        email: payload.email,
        phone: payload.phone,
        role: payload.role.map(|r| format!("{:?}", r)),
      };
      (StatusCode::CREATED, Json(ApiResponse::Success(response)))
    }
    Err(e) => {
      let msg = match e {
        sqlx::Error::Protocol(s) => s.to_string(),
        _ => "Internal server error".to_string(),
      };
      (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::Error { message: msg }),
      )
    }
  }
}
