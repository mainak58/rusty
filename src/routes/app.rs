use axum::Router;
use sqlx::PgPool;

use crate::routes::auth::auth_routes;

pub fn create_app(pool: PgPool) -> Router {
  Router::new().nest("/auth", auth_routes()).with_state(pool)
}
