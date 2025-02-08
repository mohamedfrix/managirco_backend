use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use validator::Validate;
use crate::AppState;
use crate::db::club_repo::ClubRepo;
use crate::db::collaboration_member_repo::CollaborationMemberRepo;
use crate::db::collaboration_repo::CollaborationRepo;
use crate::db::event_repo::EventRepo;
use crate::db::membership_repo::MembershipRepo;
use crate::db::userext::UserExt;
use crate::dtos::collaboration_member_dto::{AddCollaborationMemberRequestDto, AddCollaborationMemberResponseDto, CollaborationMemberDto, GetCollaborationMembersRequestDto, GetCollaborationMembersResponseDto};
use crate::error::HttpError;

pub fn collaboration_member_handler() -> Router {
    Router::new()
        .route("/add", post(add_collaboration_member))
        .route("/from_collaboration", get(get_collaboration_members))
}

pub async fn add_collaboration_member(
    Extension(app_state) : Extension<Arc<AppState>>,
    Json(body) : Json<AddCollaborationMemberRequestDto>
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

    let collaboration = app_state.db_client.get_collaboration_by_event_id(event.id, &body.collaboration_name)
        .await
        .map_err(|err| HttpError::server_error(err.to_string()))?;
    let collaboration = collaboration.ok_or(HttpError::bad_request("Collaboration does not exist"))?;

    let user = app_state.db_client.get_user(None, Some(&body.email), None)
    .await
    .map_err(|err| HttpError::server_error(err.to_string()))?;
    let user = user.ok_or( HttpError::bad_request("User does not exist"))?;

    let member = app_state.db_client.get_membership_by_user_id(user.id)
    .await
    .map_err(|err| HttpError::server_error(err.to_string()))?;
    let member = member.ok_or(HttpError::bad_request("Member does not exist"))?;

    let collaboration_role = match body.collaboration_role.as_str() {
        "Manager" => 1,
        "Member" => 2,
        _ => Err(HttpError::bad_request("Collaboration role does not exist"))?
    };

    let collaboration_member = app_state.db_client.insert_collaboration_member(member.id, collaboration.id, collaboration_role)
    .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let collaboration_member_data = CollaborationMemberDto {
        event,
        club,
        collaboration,
        member: collaboration_member
    };

    let response = AddCollaborationMemberResponseDto {
        status : "success".to_string(),
        member: collaboration_member_data
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_collaboration_members(
    Extension(app_state) : Extension<Arc<AppState>>,
    Query(params) : Query<GetCollaborationMembersRequestDto>
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

    let collaboration = app_state.db_client.get_collaboration_by_event_id(event.id, &params.collaboration_name)
    .await
    .map_err(|err| HttpError::server_error(err.to_string()))?;
    let collaboration = collaboration.ok_or(HttpError::bad_request("Collaboration does not exist"))?;

    let collaboration_members = app_state.db_client.get_collaboration_members(collaboration.id)
    .await
    .map_err(|err| HttpError::server_error(err.to_string()))?;
    let collaboration_members = collaboration_members.ok_or(HttpError::bad_request("No Members for this collaboration"))?;

    let mut collaboration_members_data = Vec::new();

    for member in collaboration_members {
        let member_data = CollaborationMemberDto {
            event: event.clone(),
            club: club.clone(),
            collaboration: collaboration.clone(),
            member: member
        };
        collaboration_members_data.push(member_data);
    }
    let response = GetCollaborationMembersResponseDto {
        status: "success".to_string(),
        members: collaboration_members_data
    };

    Ok((StatusCode::OK, Json(response)))
}
