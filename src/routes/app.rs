use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::routes::auth::auth_routes;

pub fn create_app(pool: PgPool) -> Router {
  Router::new().nest("/api", api_routes()).with_state(pool)
}

fn api_routes() -> Router<PgPool> {
  Router::new()
    .route("/ping", get(|| async { "pong" }))
    .nest("/auth", auth_routes())
}
