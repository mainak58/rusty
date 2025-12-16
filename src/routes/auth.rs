use crate::handlers::auth_handlers;
use axum::{Router, routing::post};
use sqlx::PgPool;

pub fn auth_routes() -> Router<PgPool> {
  Router::new()
    .route("/register", post(auth_handlers::create_user))
    .route("/login", post(auth_handlers::login_user))
}
