use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default, sqlx::FromRow, sqlx::Type)]
pub struct CollaborationMember {
    pub id: i32,
    pub membership_id: i32,
    pub collaboration_id: i32,
    pub collaboration_role_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}