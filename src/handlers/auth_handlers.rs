use crate::{
  config::jwt::create_jwt,
  dto::{
    auth::{
      create_user_dto::{CreateUserDto, UserResponseDto},
      login_user_dto::LoginDto,
    },
    common::common_dto::ApiResponse,
  },
  services::auth_services,
};
use axum::{
  debug_handler,
  extract::{Json, State},
  http::StatusCode,
  response::IntoResponse,
};

use serde_json::json;
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

#[debug_handler]
pub async fn login_user(
  State(pool): State<PgPool>,
  Json(payload): Json<LoginDto>,
) -> impl IntoResponse {
  let jwt_secret = std::env::var("jwt_secret").unwrap_or_else(|_| "default_secret".to_string());

  match auth_services::login_user(&pool, &payload).await {
    Ok(Some(user)) => {
      let token = create_jwt(&user, &jwt_secret).unwrap();
      (StatusCode::OK, Json(json!({ "token": token })))
    }
    Ok(None) => (
      StatusCode::UNAUTHORIZED,
      Json(json!({ "error": "Invalid credentials" })),
    ),
    Err(_) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(json!({ "error": "Something went wrong" })),
    ),
  }
}
