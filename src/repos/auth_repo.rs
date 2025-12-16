use crate::dto::auth::{create_user_dto::CreateUserDto, login_user_dto::LoginDto, user_dto::UserDto};
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

// pub async fn find_user(
//   pool: &PgPool,
//   // identifier: LoginIdentifier<'_>,
// ) -> Result<Option<UserDto>, sqlx::Error> {
//   let row = match identifier {
//     LoginIdentifier::Email(email) => {
//       sqlx::query_as::<_, UserDto>("SELECT * FROM users WHERE email = $1 LIMIT 1")
//         .bind(email)
//         .fetch_optional(pool)
//         .await?
//     }
//     LoginIdentifier::Phone(phone) => {
//       sqlx::query_as::<_, UserDto>("SELECT * FROM users WHERE phone = $1 LIMIT 1")
//         .bind(phone)
//         .fetch_optional(pool)
//         .await?
//     }
//   };

//   Ok(row)
// }
pub async fn find_user(pool: &PgPool, payload: &LoginDto) -> Result<Option<UserDto>, sqlx::Error> {
  let row = sqlx::query_as::<_, UserDto>("SELECT * FROM users WHERE email = $1 LIMIT 1")
    .bind(&payload.email)
    .fetch_optional(pool)
    .await?;

  Ok(row)
}
