use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::club_membership_role_model::ClubMembershipRole;
use crate::models::club_model::Club;
use crate::models::department_model::Department;
use crate::models::membership_model::Membership;
use crate::models::user_model::User;

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct AddMemberRequestDto {

    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email Address is invalid")
    )]
    pub email: String,
    #[validate(length(min = 1, message = "Club name is rquired"))]
    pub club_name: String,
    #[validate(length(min = 1, message = "Department name is rquired"))]
    pub department_name: String,
    #[validate(length(min = 1, message = "Role is rquired"))]
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMemberResponseDto {
    pub status: String,
    pub member: Membership
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct GetClubMembersRequestDto {
    #[validate(length(min = 1, message = "Club name is required"))]
    pub club_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct GetDepartmentMembersRequestDto {
    #[validate(length(min = 1, message = "Club name is required"))]
    pub club_name: String,

    #[validate(length(min = 1, message = "Department name is rquired"))]
    pub department_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClubMemberResponseDto {
    pub status: String,
    pub members: Vec<MemberDataResponse>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDepartmentMembersResponseDto {
    pub status: String,
    pub members: Vec<MemberDataResponse>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberDataResponse {
    pub user: User,
    pub club: Club,
    pub department: Department,
    pub role: ClubMembershipRole,
}