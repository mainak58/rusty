use crate::dto::auth::create_user_dto::CreateUserDto;
use sqlx::PgPool;

pub async fn check_user_exist(pool: &PgPool, payload: &CreateUserDto) -> Result<bool, sqlx::Error> {
  let user = sqlx::query!(
    r#"
        SELECT id FROM users
        WHERE email = $1 OR phone = $2
        "#,
    payload.email,
    payload.phone
  )
  .fetch_optional(pool)
  .await?;

  Ok(user.is_some())
}

pub async fn insert_user(pool: &PgPool, payload: &CreateUserDto) -> Result<(), sqlx::Error> {
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
