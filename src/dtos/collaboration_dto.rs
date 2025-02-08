use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::club_model::Club;
use crate::models::collaboration_model::Collaboration;
use crate::models::event_model::Event;

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct AddCollaborationRequestDto {
    #[validate(length(min = 1, message = "Club name is required"))]
    pub club_name: String,

    #[validate(length(min = 1, message = "Event name id is required"))]
    pub event_name: String,

    #[validate(length(min = 1, message = "Collaboration name is required"))]
    pub collaboration_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCollaborationResponseDto {
    pub status: String,
    pub collaboration: CollaborationDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollaborationDto {
    pub club: Club,
    pub event: Event,
    pub collaboration: Collaboration,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct GetEventCollaborationRequestDto {
    #[validate(length(min = 1, message = "Club name is required"))]
    pub club_name: String,

    #[validate(length(min = 1, message = "Event name id is required"))]
    pub event_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEventCollaborationResponseDto {
    pub status: String,
    pub collaborations: Vec<CollaborationDto>,
}