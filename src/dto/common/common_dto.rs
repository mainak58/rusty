use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
  pub message: String,
}

#[derive(Serialize)]
#[serde(tag = "status", content = "data")]
pub enum ApiResponse<T> {
  Success(T),
  Error { message: String },
}
