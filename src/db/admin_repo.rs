use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;
use crate::db::dbclient::DBClient;
use crate::models::admin_model::Admin;

#[async_trait]
pub trait AdminRepo {
    async fn insert_admin (
        &self,
        email : &str,
        name : &str,
        password : &str,
    ) -> Result<Admin, sqlx::Error>;

    async fn get_admin (
        &self,
        id: Option<Uuid>,
        email: Option<&str>,
        name: Option<&str>,
    ) -> Result<Option<Admin>, sqlx::Error>;
}

#[async_trait]
impl AdminRepo for DBClient {
    async fn insert_admin(&self, email: &str, name: &str, password: &str) -> Result<Admin, Error> {
        let admin = sqlx::query_as!(
            Admin,
            r#"
                INSERT INTO admin (name, email, password, verified) values ($1, $2, $3, $4)
                RETURNING id, email, name, password, verified, created_at, updated_at
            "#,
            name,
            email,
            password,
            false
        ).fetch_one(&self.pool).await?;

        Ok(admin)
    }

    async fn get_admin(&self, id: Option<Uuid>, email: Option<&str>, name: Option<&str>) -> Result<Option<Admin>, Error> {

        let mut admin : Option<Admin> = None;

        if let Some(id) = id {
            admin = sqlx::query_as!(
                Admin,
                r#"
                    SELECT id, email, name, password, verified, created_at, updated_at
                    FROM admin WHERE id = $1
                "#,
                id
            ).fetch_optional(&self.pool).await?;
        }
        else if let Some(email) = email {
            admin = sqlx::query_as!(
                Admin,
                r#"
                    SELECT id, email, name, password, verified, created_at, updated_at
                    FROM admin WHERE email = $1
                "#,
                email
            ).fetch_optional(&self.pool).await?;
        }
        else if let Some(name) = name {
            admin = sqlx::query_as!(
                Admin,
                r#"
                    SELECT id, email, name, password, verified, created_at, updated_at FROM admin WHERE name = $1
                "#,
                name
            ).fetch_optional(&self.pool).await?;
        }
        Ok(admin)
    }
}