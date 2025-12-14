use sqlx::{Pool, Postgres};

pub async fn create_pool() -> Pool<Postgres> {
  let db_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL not set");

  Pool::<Postgres>::connect(&db_url)
    .await
    .expect("❌ Failed to connect to database")
}
