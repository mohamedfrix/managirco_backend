use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, sqlx::Type, Clone)]

pub struct Admin {
    pub id : uuid::Uuid,
    pub email : String,
    pub name : String,
    pub password : String,
    pub verified : bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}