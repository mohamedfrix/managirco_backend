use async_trait::async_trait;
use sqlx::{Error, PgConnection};
use crate::db::dbclient::DBClient;
use crate::models::club_membership_role_model::ClubMembershipRole;

#[async_trait]
pub trait ClubMembershipRoleRepo {
    async fn add_membership_role(
        &self,
        role_name: &str,
    ) -> Result<ClubMembershipRole, sqlx::Error>;

    async fn get_all_membership_roles(
        &self,
    ) -> sqlx::Result<Vec<ClubMembershipRole>, sqlx::Error>;

    async fn get_role_by_name (
        &self,
        name: &str,
    ) -> Result<Option<ClubMembershipRole>, sqlx::Error>;

    async fn get_role_by_id (
        &self,
        id : i32
    ) -> Result<Option<ClubMembershipRole>, sqlx::Error>;
}

#[async_trait]
impl ClubMembershipRoleRepo for DBClient {
    async fn add_membership_role(&self, role_name: &str) -> Result<ClubMembershipRole, Error> {
       let result = sqlx::query_as!(
           ClubMembershipRole,
           r#"
                INSERT INTO membership_role (role_name) VALUES ($1)
                RETURNING *
            "#,
           role_name
       ).fetch_one(&self.pool)
           .await?;
        Ok(result)
    }

    async fn get_all_membership_roles(&self) -> sqlx::Result<Vec<ClubMembershipRole>, Error> {
        let result = sqlx::query_as!(
            ClubMembershipRole,
            r#"
                SELECT * FROM membership_role
            "#
        ).fetch_all(&self.pool)
            .await?;
        Ok(result)
    }

    async fn get_role_by_name(&self, name: &str) -> Result<Option<ClubMembershipRole>, Error> {

        let result = sqlx::query_as!(
            ClubMembershipRole,
            r#"
                SELECT * FROM membership_role WHERE role_name = $1
            "#,
            name
        ).fetch_one(&self.pool)
        .await?;

        Ok(Some(result))
    }

    async fn get_role_by_id(&self, id: i32) -> Result<Option<ClubMembershipRole>, Error> {
        let result = sqlx::query_as!(
            ClubMembershipRole,
            r#"
                SELECT * FROM membership_role WHERE id = $1
            "#,
            id
        ).fetch_one(&self.pool)
            .await?;

        Ok(Some(result))
    }
}