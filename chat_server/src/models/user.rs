use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::prelude::FromRow;

use crate::AppError;

use super::User;

#[derive(Debug, FromRow)]
struct EmailPass {
    email: String,
    password_hash: String,
}

impl User {
    pub async fn find_by_email(email: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
        let user =
            sqlx::query_as("SELECT id, fullname, email, created_at FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(pool)
                .await?;

        Ok(user)
    }

    pub async fn create(
        email: &str,
        fullname: &str,
        password: &str,
        pool: &sqlx::PgPool,
    ) -> Result<Self, AppError> {
        let password_hash = hash_password(password)?;

        let user = sqlx::query_as(
            "INSERT INTO users (email, fullname, password_hash) 
                VALUES ($1, $2, $3)
                RETURNING id, fullname, email, created_at",
        )
        .bind(email)
        .bind(fullname)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn verify(
        email: &str,
        password: &str,
        pool: &sqlx::PgPool,
    ) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, fullname, email, password_hash, created_at FROM users where email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;
        match user {
            Some(mut user) => {
                let password_hash = std::mem::take(&mut user.password_hash);
                let is_valid = verify_password(password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash)?;

    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}
