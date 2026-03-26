use async_trait::async_trait;

use domain::{
    shared::{repository::error::RepositoryError, value_objects::password_hash::PasswordHash},
    user::{
        User,
        repository::UserRepository,
        value_objects::{email::Email, id::UserId},
    },
};
use sqlx::{Pool, Postgres};

use crate::database::postgres::error::map_sqlx_error_to_domain_error;

pub struct PostgresUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, user: &User) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
                INSERT INTO users (id, email, password_hash, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id().uuid(),
            user.email().as_str(),
            user.password_hash().as_str(),
            user.created_at(),
            user.updated_at()
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        Ok(())
    }

    async fn get_password_hash_by_email(
        &self,
        email: &Email,
    ) -> Result<PasswordHash, RepositoryError> {
        let record = sqlx::query!(
            r#"
                SELECT password_hash
                FROM users
                WHERE email = $1
            "#,
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        match record {
            Some(row) => Ok(PasswordHash::new(row.password_hash)),
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn get_user_id_by_email(&self, email: &Email) -> Result<UserId, RepositoryError> {
        let record = sqlx::query!(
            r#"
                SELECT id
                FROM users
                WHERE email = $1
            "#,
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error_to_domain_error)?;

        match record {
            Some(row) => Ok(UserId::new(row.id)),
            None => Err(RepositoryError::NotFound),
        }
    }
}
