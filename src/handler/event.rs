use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use validator::Validate;
use crate::AppState;
use crate::db::club_repo::ClubRepo;
use crate::db::event_repo::EventRepo;
use crate::dtos::event_dto::{AddEventRequestDto, AddEventResponseDto, EventDto, GetClubEventsRequestDto, GetClubEventsResponseDto};
use crate::error::HttpError;

pub fn event_handler () -> Router {
    Router::new()
        .route("/", post(add_event))
        .route("/from_club", get(get_club_events))
}

pub async fn add_event (
    Extension(app_state) : Extension<Arc<AppState>>,
    Json(body) : Json<AddEventRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let club = app_state.db_client.get_club_by_name(&body.club_name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let event = app_state.db_client.insert_event(club.id, &body.event_name, &body.event_description, body.start_date, body.end_date)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let event_data = EventDto {
        club: club,
        event: event,
    };

    let response = AddEventResponseDto{
        status : "success".to_string(),
        event: event_data,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_club_events(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params) : Query<GetClubEventsRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    params.validate()
    .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let club = app_state.db_client.get_club_by_name(&params.club_name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let events = app_state.db_client.get_events_by_club_id(club.id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let events = events.ok_or(HttpError::bad_request("Event does not exist"))?;

    let mut events_data = Vec::new();
    for event in events {
        let event_data = EventDto{
            club: club.clone(),
            event: event
        };
        events_data.push(event_data);
    }
    let response = GetClubEventsResponseDto{
        status: "success".to_string(),
        events: events_data
    };

    Ok((StatusCode::OK, Json(response)))

}