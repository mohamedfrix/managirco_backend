use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::db::dbclient::DBClient;
use crate::models::user_model::{User};

#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        email: Option<&str>,
        token: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;

    async fn get_users(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<User>, sqlx::Error>;

    async fn save_user<T: Into<String> + Send>(
        &self,
        first_name: T,
        last_name: T,
        email: T,
        phone_number: T,
        password: T,
        verification_token: T,
        token_expires_at: DateTime<Utc>,
    ) -> Result<User, sqlx::Error>;

    async fn get_user_count(&self) -> Result<i64, sqlx::Error>;

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        first_name: T,
        last_name: T,
    ) -> Result<User, sqlx::Error>;

    // async fn update_user_role(
    //     &self,
    //     user_id: Uuid,
    //     role: UserRole,
    // ) -> Result<User, sqlx::Error>;

    async fn update_user_password(
        &self,
        user_id: Uuid,
        password: String,
    ) -> Result<User, sqlx::Error>;

    async fn verifed_token(
        &self,
        token: &str,
    ) -> Result<(), sqlx::Error>;

    async fn add_verifed_token(
        &self,
        user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl UserExt for DBClient {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        email: Option<&str>,
        token: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        let mut user: Option<User> = None;

        if let Some(user_id) = user_id {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, first_name, last_name, email, phone_number,password, verified, created_at, updated_at, verification_token, token_expires_at FROM users WHERE id = $1"#,
                user_id
            ).fetch_optional(&self.pool).await?;
        // } else if let Some(name) = name {
        //     user = sqlx::query_as!(
        //         User,
        //         r#"SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role as "role: UserRole" FROM users WHERE name = $1"#,
        //         name
        //     ).fetch_optional(&self.pool).await?;
        } else if let Some(email) = email {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, first_name, last_name, email, phone_number,password, verified, created_at, updated_at, verification_token, token_expires_at FROM users WHERE email = $1"#,
                email
            ).fetch_optional(&self.pool).await?;
        } else if let Some(token) = token {
            user = sqlx::query_as!(
                User,
                r#"
                SELECT id, first_name, last_name, email, phone_number, password, verified, created_at, updated_at, verification_token, token_expires_at
                FROM users
                WHERE verification_token = $1"#,
                token
            )
                .fetch_optional(&self.pool)
                .await?;
        }

        Ok(user)
    }

    async fn get_users(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let users = sqlx::query_as!(
            User,
            r#"SELECT * FROM users
            ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64,
        ).fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn save_user<T: Into<String> + Send>(
        &self,
        first_name: T,
        last_name: T,
        email: T,
        phone_number: T,
        password: T,
        verification_token: T,
        token_expires_at: DateTime<Utc>,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (first_name, last_name, email, phone_number, password, verification_token, token_expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, first_name, last_name, email, phone_number, password, verified, created_at, updated_at, verification_token, token_expires_at
            "#,
            first_name.into(),
            last_name.into(),
            email.into(),
            phone_number.into(),
            password.into(),
            verification_token.into(),
            token_expires_at
        ).fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn get_user_count(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM users"#
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(count.unwrap_or(0))
    }

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        new_first_name: T,
        new_last_name: T
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET first_name = $1, last_name = $2, updated_at = Now()
            WHERE id = $3
            RETURNING id, first_name, last_name, email, phone_number, password, verified, created_at, updated_at, verification_token, token_expires_at
            "#,
            new_first_name.into(),
            new_last_name.into(),
            user_id
        ).fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    // async fn update_user_role(
    //     &self,
    //     user_id: Uuid,
    //     new_role: UserRole
    // ) -> Result<User, sqlx::Error> {
    //     let user = sqlx::query_as!(
    //         User,
    //         r#"
    //         UPDATE users
    //         SET role = $1, updated_at = Now()
    //         WHERE id = $2
    //         RETURNING id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role as "role: UserRole"
    //         "#,
    //         new_role as UserRole,
    //         user_id
    //     ).fetch_one(&self.pool)
    //         .await?;
    //
    //     Ok(user)
    // }

    async fn update_user_password(
        &self,
        user_id: Uuid,
        new_password: String
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, first_name, last_name, email, phone_number, password, verified, created_at, updated_at, verification_token, token_expires_at
            "#,
            new_password,
            user_id
        ).fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn verifed_token(
        &self,
        token: &str,
    ) -> Result<(), sqlx::Error> {
        let _ =sqlx::query!(
            r#"
            UPDATE users
            SET verified = true,
                updated_at = Now(),
                verification_token = NULL,
                token_expires_at = NULL
            WHERE verification_token = $1
            "#,
            token
        ).execute(&self.pool)
            .await;

        Ok(())
    }

    async fn add_verifed_token(
        &self,
        user_id: Uuid,
        token: &str,
        token_expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            r#"
            UPDATE users
            SET verification_token = $1, token_expires_at = $2, updated_at = Now()
            WHERE id = $3
            "#,
            token,
            token_expires_at,
            user_id,
        ).execute(&self.pool)
            .await?;

        Ok(())
    }
}