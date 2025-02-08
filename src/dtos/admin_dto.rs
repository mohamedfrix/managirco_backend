use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::admin_model::Admin;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterAdminDtos {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is not valid")
    )]
    pub email : String,

    #[validate(length(min = 1, message = "Name is required"))]
    pub name : String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 8, message = "Password should be at least 8 characters")
    )]
    pub password : String,

    #[validate(
        length(min = 1, message = "password_confirm is required"),
        must_match(other = "password", message = "password_confirm must match password")
    )]
    pub password_confirm : String,
}


#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginAdminRequestDto {
    #[validate(
        length(min = 1, message = "email required"),
        email(message = "Email is not valid")
    )]
    pub email : String,

    #[validate(length(min = 1, message = "password required"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAdminResponseDto {
    pub status: String,
    pub admin: Admin,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAdminResponseDto {
    pub status: String,
    pub admin: Admin,
    pub token: String,
}