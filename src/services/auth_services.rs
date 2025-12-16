use crate::dto::auth::create_user_dto::CreateUserDto;
use crate::dto::auth::login_user_dto::{LoginDto, LoginIdentifier};
use crate::dto::auth::user_dto::UserDto;
use crate::repos::auth_repo::{self, find_user};
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, payload: &CreateUserDto) -> Result<(), sqlx::Error> {
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

pub fn resolve_identifier(dto: &UserDto) -> anyhow::Result<LoginIdentifier<'_>> {
  if let Some(email) = &dto.email {
    return Ok(LoginIdentifier::Email(email));
  }

  if let Some(phone) = &dto.phone {
    return Ok(LoginIdentifier::Phone(phone));
  }

  Err(anyhow::anyhow!("No login identifier provided"))
}

// pub async fn login_user(
//   pool: &PgPool,
//   // identifier: LoginIdentifier<'_>,
//   // password: &str,
//   payload: &UserDto,
// ) -> anyhow::Result<Option<UserDto>> {
//   // let user = find_user(pool, identifier).await?;
//   let user = find_user(pool).await?;

//   if let Some(u) = user {
//     if u.password == &payload.password {
//       Ok(Some(u))
//     } else {
//       Ok(None)
//     }
//   } else {
//     Ok(None)
//   }
// }

pub async fn login_user(pool: &PgPool, payload: &LoginDto) -> anyhow::Result<Option<UserDto>> {
  let user = find_user(pool, payload).await?;

  if let Some(u) = user {
    if u.password == payload.password {
      Ok(Some(u))
    } else {
      Ok(None)
    }
  } else {
    Ok(None)
  }
}
