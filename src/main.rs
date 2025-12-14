use backend::db::pool::create_pool;
use backend::routes::app::create_app;
use dotenvy::dotenv;
use std::env;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
  dotenv().ok();
  fmt::init();

  let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
  let addr = format!("0.0.0.0:{}", port);

  let pool = create_pool().await;
  println!("Connected to Postgres!");

  let app = create_app(pool);
  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .expect("Failed to listen to port");

  println!("Server running at http://localhost:{port}");

  axum::serve(listener, app).await.expect("Server error");
}
