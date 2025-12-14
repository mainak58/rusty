use axum::Json;
use sqlx::{Pool, Postgres};

pub async fn health() -> &'static str {
  "Axum backend running!"
}

pub async fn test_db(pool: Pool<Postgres>) -> Json<String> {
  let row: (i64,) = sqlx::query_as("SELECT 1000")
    .fetch_one(&pool)
    .await
    .unwrap();

  Json(format!("DB says: {}", row.0))
}
