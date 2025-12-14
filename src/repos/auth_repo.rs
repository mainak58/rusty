use crate::dto::auth::create_user_dto::CreateUserDto;
use sqlx::PgPool;

pub async fn insert_user(pool: &PgPool, payload: CreateUserDto) -> Result<(), sqlx::Error> {
  sqlx::query!(
    r#"
    INSERT INTO users (name, email, phone, password)
    VALUES ($1, $2, $3, $4)
    "#,
    payload.name,
    payload.email,
    payload.phone,
    payload.password,
  )
  .execute(pool)
  .await?;

  Ok(())
}
