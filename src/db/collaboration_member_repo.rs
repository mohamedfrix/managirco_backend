use async_trait::async_trait;
use sqlx::Error;
use crate::db::dbclient::DBClient;
use crate::models::collaboration_member_model::CollaborationMember;

#[async_trait]
pub trait CollaborationMemberRepo {
    async fn insert_collaboration_member(
        &self,
        membership_id: i32,
        collaboration_id: i32,
        collaboration_role_id: i32
    ) -> Result<CollaborationMember, sqlx::Error>;

    async fn get_collaboration_members(
        &self,
        collaboration_id: i32,
    ) -> Result<Option<Vec<CollaborationMember>>, sqlx::Error>;

}

#[async_trait]
impl CollaborationMemberRepo for DBClient {
    async fn insert_collaboration_member(&self, membership_id: i32, collaboration_id: i32, collaboration_role_id: i32) -> Result<CollaborationMember, Error> {
        let result = sqlx::query_as!(
            CollaborationMember,
            r#"
                INSERT INTO collaboration_members (membership_id, collaboration_id, collaboration_role_id) VALUES ($1,$2, $3) RETURNING *
            "#,
            membership_id,
            collaboration_id,
            collaboration_role_id
        ).fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_collaboration_members(&self, collaboration_id: i32) -> Result<Option<Vec<CollaborationMember>>, Error> {
        let result = sqlx::query_as!(
            CollaborationMember,
            r#"
                SELECT * FROM collaboration_members WHERE collaboration_id = $1 ORDER BY created_at DESC
            "#,
            collaboration_id
        ).fetch_all(&self.pool)
        .await?;

        Ok(Some(result))
    }
}