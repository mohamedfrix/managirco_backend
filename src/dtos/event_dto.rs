use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::models::club_model::Club;
use crate::models::event_model::Event;

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct AddEventRequestDto {
    #[validate(
        length(min = 1, message = "Club name is required"),
    )]
    pub club_name: String,

    #[validate(length(min = 1, message = "Event name is required"))]
    pub event_name: String,

    #[validate(
        length(min = 1, message = "Event description is required"),
        length(min = 10, message = "Event description is too short")
    )]
    pub event_description: String,

    #[validate(custom = "validate_start_date")]
    pub start_date: DateTime<Utc>,

    #[validate(custom = "validate_end_date")]
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddEventResponseDto {
    pub status: String,
    pub event: EventDto
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventDto {
    pub club: Club,
    pub event: Event
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct GetClubEventsRequestDto {

    #[validate(length(min = 1, message = "Club Name is required"))]
    pub club_name : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClubEventsResponseDto {
    pub status: String,
    pub events: Vec<EventDto>
}


fn validate_start_date(date: &DateTime<Utc>) -> Result<(), ValidationError> {
    if *date < Utc::now() {
        return Err(ValidationError::new("start_date must be in the future"));
    }
    Ok(())
}

fn validate_end_date(end_date: &DateTime<Utc>) -> Result<(), ValidationError> {
    if *end_date < Utc::now() {
        return Err(ValidationError::new("end_date must be in the future"));
    }
    Ok(())
}