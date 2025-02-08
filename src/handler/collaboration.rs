use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use validator::Validate;
use crate::AppState;
use crate::db::club_repo::ClubRepo;
use crate::db::collaboration_repo::CollaborationRepo;
use crate::db::event_repo::EventRepo;
use crate::dtos::collaboration_dto::{AddCollaborationRequestDto, AddCollaborationResponseDto, CollaborationDto, GetEventCollaborationRequestDto, GetEventCollaborationResponseDto};
use crate::error::HttpError;

pub fn collaboration_handler() -> Router {
    Router::new()
        .route("/add", post(add_collaboration))
        .route("/from_event", get(get_event_collaboration))
}

async fn add_collaboration(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<AddCollaborationRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    body.validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let club = app_state.db_client.get_club_by_name(&body.club_name)
        .await
        .map_err(|err| HttpError::server_error(err.to_string()))?;

    let event = app_state.db_client.get_event_by_club_id(club.id, &body.event_name)
        .await
        .map_err(|err| HttpError::server_error(err.to_string()))?;
    let event = event.ok_or(HttpError::bad_request("Event does not exist"))?;

    let collaboration = app_state.db_client.insert_collaboration(event.id, &body.collaboration_name)
    .await
    .map_err(|err| HttpError::server_error(err.to_string()))?;

    let collaboration_data = CollaborationDto {
        club,
        event,
        collaboration
    };

    let response = AddCollaborationResponseDto {
        status: "successs".to_string(),
        collaboration: collaboration_data
    };

    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_event_collaboration(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params) : Query<GetEventCollaborationRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    params.validate()
    .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let club = app_state.db_client.get_club_by_name(&params.club_name)
    .await
    .map_err(|err| HttpError::server_error(err.to_string()))?;

    let event = app_state.db_client.get_event_by_club_id(club.id, &params.event_name)
    .await
    .map_err(|err| HttpError::server_error(err.to_string()))?;
    let event = event.ok_or(HttpError::bad_request("Event does not exist"))?;

    let collaborations = app_state.db_client.get_event_collaborations(event.id)
        .await
    .map_err(|err| HttpError::server_error(err.to_string()))?;

    let collaborations = collaborations.ok_or(HttpError::bad_request("No Collaborations for this event"))?;

    let mut collaborations_data = Vec::new();
    for collaboration in collaborations {
        let collaboration_data = CollaborationDto {
            club: club.clone(),
            event: event.clone(),
            collaboration
        };
        collaborations_data.push(collaboration_data);
    }
    let response = GetEventCollaborationResponseDto{
        status: "success".to_string(),
        collaborations: collaborations_data
    };

    Ok((StatusCode::OK, Json(response)))
}