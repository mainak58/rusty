use backend::create_router;
use backend::db::pool::create_pool;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
  dotenv().ok();

  let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
  let addr = format!("0.0.0.0:{}", port);

  let pool = create_pool().await;
  println!("✅ Connected to Postgres!");

  let app = create_router(pool);

  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .expect("❌ Failed to bind port");

  println!("🚀 Server running at http://localhost:{port}");

  axum::serve(listener, app).await.expect("❌ Server error");
}
