pub mod db;
pub mod dto;
pub mod handlers;
pub mod models;
pub mod repos;
pub mod routes;
pub mod services;

use axum::{Json, Router, routing::get};
use sqlx::{Pool, Postgres};

pub async fn health() -> &'static str {
  "Server is running!"
}

pub async fn test_db(pool: Pool<Postgres>) -> Json<String> {
  let row: (i64,) = sqlx::query_as("SELECT 123").fetch_one(&pool).await.unwrap();

  Json(format!("DB OK: {}", row.0))
}

pub fn create_router(pool: Pool<Postgres>) -> Router {
  let pool_clone = pool.clone();

  Router::new()
    .route("/", get(health))
    .route("/db", get(move || test_db(pool_clone.clone())))
}
