use async_trait::async_trait;
use sqlx::Error;
use crate::db::dbclient::DBClient;
use crate::models::collaboration_model::Collaboration;

#[async_trait]
pub trait CollaborationRepo {
    async fn insert_collaboration(
        &self,
        event_id: i32,
        name: &str,
    ) -> Result<Collaboration, sqlx::Error>;

    async fn get_event_collaborations(
        &self,
        event_id: i32,
    ) -> Result<Option<Vec<Collaboration>>, sqlx::Error>;

    async fn get_collaboration_by_event_id(
        &self,
        event_id: i32,
        name: &str
    ) -> Result<Option<Collaboration>, sqlx::Error>;
}

#[async_trait]
impl CollaborationRepo for DBClient {
    async fn insert_collaboration(&self, event_id: i32, name: &str) -> Result<Collaboration, Error> {
        let result = sqlx::query_as!(
            Collaboration,
            r#"
                INSERT INTO collaboration (event_id, name) VALUES ($1, $2)
                RETURNING *
            "#,
            event_id,
            name
        ).fetch_one(&self.pool)
            .await?;
        Ok(result)
    }

    async fn get_event_collaborations(&self, event_id: i32) -> Result<Option<Vec<Collaboration>>, Error> {
        let result = sqlx::query_as!(
            Collaboration,
            r#"
                SELECT * FROM collaboration WHERE event_id = $1
            "#,
            event_id
        ).fetch_all(&self.pool)
        .await?;

        Ok(Some(result))
    }

    async fn get_collaboration_by_event_id(&self, event_id: i32, name: &str) -> Result<Option<Collaboration>, Error> {
        let result = sqlx::query_as!(
            Collaboration,
            r#"
                SELECT * FROM collaboration WHERE event_id = $1 AND name = $2
            "#,
            event_id,
            name
        ).fetch_one(&self.pool)
            .await?;

        Ok(Some(result))
    }
}