use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::club_model::Club;
use crate::models::collaboration_member_model::CollaborationMember;
use crate::models::collaboration_model::Collaboration;
use crate::models::event_model::Event;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollaborationMemberDto {
    pub event: Event,
    pub club: Club,
    pub collaboration: Collaboration,
    pub member: CollaborationMember,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct AddCollaborationMemberRequestDto {
    #[validate(length(min = 1, message = "Club name is required"))]
    pub club_name: String,

    #[validate(length(min = 1, message = "Event name is required"))]
    pub event_name: String,

    #[validate(length(min = 1, message = "Collaboration name is required"))]
    pub collaboration_name: String,

    #[validate(length(min = 1, message = "Collaboration role is required"))]
    pub collaboration_role: String,

    #[validate(
        length(min = 1, message = "Member email is required"),
        email(message = "Email address is not valid")
    )]
    pub email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCollaborationMemberResponseDto {
    pub status : String,
    pub member : CollaborationMemberDto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct GetCollaborationMembersRequestDto {
    #[validate(length(min = 1, message = "Club nameis required"))]
    pub club_name: String,

    #[validate(length(min = 1, message = "Event name is required"))]
    pub event_name: String,

    #[validate(length(min = 1, message = "Collaboration name is required"))]
    pub collaboration_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCollaborationMembersResponseDto {
    pub status : String,
    pub members: Vec<CollaborationMemberDto>,
}