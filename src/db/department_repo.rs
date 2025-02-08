use async_trait::async_trait;
use sqlx::Error;
use crate::db::club_repo::ClubRepo;
use crate::db::dbclient::DBClient;
use crate::error::HttpError;
use crate::models::club_model::Club;
use crate::models::department_model::Department;

#[async_trait]
pub trait DepartmentRepo {
  async fn insert_department(
      &self,
      club_name: &str,
      department_name: &str,
  )  -> Result<Department, sqlx::Error>;

    async fn find_departments_by_club(
        &self,
        club_name: &str,
    ) -> Result<Vec<Department>, sqlx::Error>;

    async fn get_department_by_name (
        &self,
        club_id: i32,
        department_name: &str,
    ) -> Result<Department, sqlx::Error>;
}

#[async_trait]
impl DepartmentRepo for DBClient {
    async fn insert_department(&self, club_name: &str, department_name: &str) -> Result<Department, Error> {
        let club = DBClient::get_club_by_name(&self, club_name).await?;
        let department = sqlx::query_as!(
                    Department,
                    r#"
                        INSERT INTO department (club_id, name) values ($1, $2)
                        RETURNING *
                    "#,
                    club.id,
                    department_name
                ).fetch_one(&self.pool)
            .await?;
        Ok(department)
    }

    async fn find_departments_by_club(&self, club_name: &str) -> Result<Vec<Department>, Error> {
        let club = DBClient::get_club_by_name(&self, club_name).await?;
        let departments = sqlx::query_as!(
                    Department,
                    r#"
                        SELECT * FROM department WHERE club_id = $1 ORDER BY created_at DESC
                    "#,
                    club.id
                ).fetch_all(&self.pool)
            .await?;
        Ok(departments)
    }

    async fn get_department_by_name(&self, club_id: i32, department_name: &str) -> Result<Department, Error> {
        let result = sqlx::query_as!(
            Department,
            r#"
                SELECT * FROM department WHERE club_id = $1 AND name = $2
            "#,
            club_id,
            department_name
        ).fetch_one(&self.pool)
            .await?;
        Ok(result)
    }
}