use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::club_model::Club;

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct AddClubDto{
    #[validate(
        length(min = 1, message = "Club name is required"),
        length(max = 40, message = "Club name is too large"),
    )]
    pub name: String,

    #[validate(length(min = 1, message = "school name is required"))]
    pub school : String,

    #[validate(
        length(min = 1, message = "Club email is required"),
        email(message = "Email is invalid"),
    )]
    pub email: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ClubResponseDto {
    pub status: String,
    pub club: Club,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllClubsResponseDto {
    pub status: String,
    pub clubs: Vec<Club>,
}