use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
// #[sqlx(type_name = "user_role", rename_all = "lowercase")]
// pub enum UserRole {
//     Admin,
//     User
// }
//
// impl UserRole {
//     pub fn to_str(&self) -> &str {
//         match self {
//             UserRole::Admin => "admin",
//             UserRole::User => "user",
//         }
//     }
// }

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, sqlx::Type, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub verified: bool,
    pub verification_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,

}