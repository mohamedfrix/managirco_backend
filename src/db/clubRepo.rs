use async_trait::async_trait;
use sqlx::Error;
use crate::db::dbclient::DBClient;
use crate::models::club_model::Club;

#[async_trait]
pub trait ClubRepo {
    async fn insert_club(
        &self,
        name: &str,
        school: &str,
        email: &str,
    ) -> Result<Club, sqlx::Error>;

    async fn get_all_clubs (
        &self,
    ) -> Result<Vec<Club>, sqlx::Error>;

    async fn get_club_name (
        &self,
        name: &str,
    ) -> Result<Club, sqlx::Error>;

}

#[async_trait]
impl ClubRepo for DBClient {
    async fn insert_club(&self, name: &str, school: &str, email: &str) -> Result<Club, Error> {
        let club = sqlx::query_as!(
            Club,
            r#"
            INSERT INTO club (name, school, email)
            VALUES ($1, $2, $3)
            RETURNING club_id, name, school, email, created_at, updated_at
            "#,
            name,
            school,
            email
        ).fetch_one(&self.pool)
        .await?;

        Ok(club)
    }

    async fn get_all_clubs(&self) -> Result<Vec<Club>, Error> {
        let clubs = sqlx::query_as!(
            Club,
            r#"
                SELECT * FROM club
                ORDER BY created_at DESC
            "#
        ).fetch_all(&self.pool)
            .await?;
        Ok(clubs)
    }

    async fn get_club_name(&self, name: &str) -> Result<Club, Error> {
        todo!()
    }
}

