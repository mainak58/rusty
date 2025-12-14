pub mod db;
pub mod dto;
pub mod handlers;
pub mod models;
pub mod repos;
pub mod routes;
pub mod services;
pub async fn health() -> &'static str {
  "Server is running!"
}
