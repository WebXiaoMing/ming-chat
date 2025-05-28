use crate::utils::auth::{hash_password, verify_password};
use crate::{error::AppError, models::user::*};
use sqlx::{PgPool, query_as};

impl User {
    #[allow(dead_code)]
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user = query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn signup(create_user: CreateUser, pool: &PgPool) -> Result<Self, AppError> {
        let parsed_hash = hash_password(&create_user.hashed_password)?;
        let user = query_as(
            r#"
                INSERT INTO users (fullname, email, hashed_password, avatar)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
        )
        .bind(&create_user.fullname)
        .bind(&create_user.email)
        .bind(&parsed_hash)
        .bind(create_user.avatar.unwrap_or_default())
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn signin(
        email: &str,
        hashed_password: &str,
        pool: &PgPool,
    ) -> Result<Option<Self>, AppError> {
        let user: Option<User> = query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
        if let Some(user) = user {
            if verify_password(hashed_password, &user.hashed_password)? {
                return Ok(Some(user));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::{
        error::AppError,
        models::user::{CreateUser, User},
    };

    #[sqlx::test(migrations = "../migrations")]
    async fn signup_user_should_work(pool: PgPool) -> Result<(), AppError> {
        let create_user = CreateUser {
            fullname: "xiaoming".to_string(),
            email: "123@321.qq.com".to_string(),
            avatar: None,
            hashed_password: "888".to_string(),
        };
        let user = User::signup(create_user.clone(), &pool).await?;
        assert_eq!(user.email, create_user.email);
        assert_eq!(user.fullname, create_user.fullname);
        assert_ne!(user.id, 0);
        Ok(())
    }
}
