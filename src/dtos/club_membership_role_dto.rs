use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::club_membership_role_model::ClubMembershipRole;

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate )]
pub struct AddClubMembershipRoleRequestDto {
    #[validate(length(min = 1, message = "Role name is required"))]
    pub role_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddClubMembershipRoleResponseDto {
    pub status: String,
    pub membership_role: ClubMembershipRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllClubMembershipRolesResponse {
    pub status: String,
    pub membership_roles: Vec<ClubMembershipRole>,
}