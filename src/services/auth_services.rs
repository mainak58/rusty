use crate::dto::auth::create_user_dto::CreateUserDto;
use crate::repos::auth_repo;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, payload: &CreateUserDto) -> Result<(), sqlx::Error> {
  tracing::debug!("{:#?}", payload);

  if payload.name.trim().is_empty() {
    return Err(sqlx::Error::Protocol("Name cannot be empty".into()));
  }

  if payload.email.is_none() && payload.phone.is_none() {
    return Err(sqlx::Error::Protocol(
      "Either email or phone must be provided".into(),
    ));
  }

  if payload.password.trim().is_empty() {
    return Err(sqlx::Error::Protocol("Password cannot be empty".into()));
  }

  if auth_repo::check_user_exist(pool, payload).await? {
    return Err(sqlx::Error::Protocol("User already exists".into()));
  }

  auth_repo::insert_user(pool, payload).await?;

  Ok(())
}
