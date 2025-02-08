use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Error;
use crate::db::dbclient::DBClient;
use crate::models::event_model::Event;

#[async_trait]
pub trait EventRepo {
    async fn insert_event(
        &self,
        club_id: i32,
        name: &str,
        description: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Event, sqlx::Error>;

    async fn get_events_by_club_id (
        &self,
        club_id: i32
    ) -> Result<Option<Vec<Event>>, sqlx::Error>;

    async fn get_event_by_club_id (
        &self,
        club_id: i32,
        event_name: &str
    ) -> Result<Option<Event>, sqlx::Error>;

}

#[async_trait]
impl EventRepo for DBClient {
    async fn insert_event(&self, club_id: i32, name: &str, description: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Event, Error> {
        let result = sqlx::query_as!(
            Event,
            r#"
                INSERT INTO event (club_id, name, description, start_date, end_date) VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            "#,
            club_id,
            name,
            description,
            start_date,
            end_date
        ).fetch_one(&self.pool).await?;

        Ok(result)
    }

    async fn get_events_by_club_id(&self, club_id: i32) -> Result<Option<Vec<Event>>, Error> {
        let result = sqlx::query_as!(
            Event,
            r#"
                SELECT * FROM event WHERE club_id = $1 ORDER BY start_date DESC
            "#,
            club_id
        ).fetch_all(&self.pool)
            .await?;

        Ok(Some(result))
    }

    async fn get_event_by_club_id(&self, club_id: i32, event_name: &str) -> Result<Option<Event>, Error> {
        let result = sqlx::query_as!(
            Event,
            r#"
                SELECT * FROM event WHERE club_id = $1 AND name = $2
            "#,
            club_id,
            event_name
        ).fetch_one(&self.pool)
            .await?;
        Ok(Some(result))
    }
}