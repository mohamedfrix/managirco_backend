use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;
use crate::db::dbclient::DBClient;
use crate::models::membership_model::Membership;

#[async_trait]
pub trait MembershipRepo {
    async fn add_member(
        &self,
        user_id: Uuid,
        department_id: i32,
        role_id: i32,
    ) -> Result<Membership, sqlx::Error>;

    async fn get_department_members(
        &self,
        department_id: i32,
    ) -> Result<Vec<Membership>, sqlx::Error>;

    async fn get_membership_by_user_id (
        &self,
        user_id: Uuid,
    )-> Result<Option<Membership>, sqlx::Error>;

}

#[async_trait]
impl MembershipRepo for DBClient {
    async fn add_member(&self, user_id: Uuid, department_id: i32, role_id: i32) -> Result<Membership, Error> {
        let result = sqlx::query_as!(
            Membership,
            r#"
                INSERT INTO membership (user_id, department_id, role_id) VALUES ($1, $2, $3)
                RETURNING *
            "#,
            user_id,
            department_id,
            role_id,
        ).fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_department_members(&self, department_id: i32) -> Result<Vec<Membership>, Error> {
        let result = sqlx::query_as!(
            Membership,
            r#"
                SELECT * FROM membership WHERE department_id = $1
            "#,
            department_id,
        ).fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    async fn get_membership_by_user_id(&self, user_id: Uuid) -> Result<Option<Membership>, Error> {
        let result = sqlx::query_as!(
            Membership,
            r#"
                SELECT * FROM membership WHERE user_id = $1
            "#,
            user_id,
        ).fetch_one(&self.pool)
        .await?;

        Ok(Some(result))
    }
}