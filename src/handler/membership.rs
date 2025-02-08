use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use validator::Validate;
use crate::AppState;
use crate::db::club_membership_role_repo::ClubMembershipRoleRepo;
use crate::db::club_repo::ClubRepo;
use crate::db::department_repo::DepartmentRepo;
use crate::db::membership_repo::MembershipRepo;
use crate::db::userext::UserExt;
use crate::dtos::mambership_dto::{AddMemberRequestDto, AddMemberResponseDto, GetClubMemberResponseDto, GetClubMembersRequestDto, GetDepartmentMembersRequestDto, GetDepartmentMembersResponseDto, MemberDataResponse};
use crate::error::HttpError;

pub fn membership_handler () -> Router {
    Router::new()
        .route("/add", post(add_member))
        .route("/from_club", get(get_club_members))
        .route("/from_department", get(get_department_members))
}

pub async fn add_member(
    Extension(app_state) : Extension<Arc<AppState>>,
    Json(body) : Json<AddMemberRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = app_state.db_client.get_user(None, Some(&body.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = user.ok_or(HttpError::bad_request("User does not exist"))?;

    let club = app_state.db_client.get_club_by_name(&body.club_name)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let department = app_state.db_client.get_department_by_name(club.id, &body.department_name)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let role = app_state.db_client.get_role_by_name(&body.role)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let role = role.ok_or(HttpError::bad_request("Role does not exist"))?;

    let member = app_state.db_client.add_member(user.id, department.id, role.id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = AddMemberResponseDto {
        status: "success".to_string(),
        member: member,
    };
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_club_members(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params) : Query<GetClubMembersRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    params.validate()
    .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let club = app_state.db_client.get_club_by_name(&params.club_name)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let departments = app_state.db_client.find_departments_by_club(&club.name)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut members = Vec::new();

    for department in departments {
        let department_members = app_state.db_client.get_department_members(department.id)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;
        // members.extend(department_members);
        let mut response_department_members = Vec::new();
        for member in department_members {
            let role = app_state.db_client.get_role_by_id(member.role_id)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;
            let role = role.ok_or(HttpError::bad_request("Role does not exist"))?;

            let user = app_state.db_client.get_user(Some(member.user_id), None, None)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;
            let user = user.ok_or(HttpError::bad_request("User does not exist"))?;

            let club_member = MemberDataResponse {
                user: user,
                club: club.clone(),
                department: department.clone(),
                role: role.clone(),
            };
            response_department_members.push(club_member);
        }
        members.extend(response_department_members);
    }

    let response = GetClubMemberResponseDto{
        status: "success".to_string(),
        members: members,
    };
    Ok((StatusCode::OK, Json(response)))
}

pub async fn get_department_members(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params) : Query<GetDepartmentMembersRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    params.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let club = app_state.db_client.get_club_by_name(&params.club_name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let department = app_state.db_client.get_department_by_name(club.id, &params.department_name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let members = app_state.db_client.get_department_members(department.id)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut members_data = Vec::new();

    for member in members {
        let role = app_state.db_client.get_role_by_id(member.role_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
        let role = role.ok_or(HttpError::bad_request("Role does not exist"))?;

        let user = app_state.db_client.get_user(Some(member.user_id), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
        let user = user.ok_or(HttpError::bad_request("User does not exist"))?;

        let department_member = MemberDataResponse {
            user: user,
            club: club.clone(),
            department: department.clone(),
            role: role,
        };
        members_data.push(department_member);
    }

    let response = GetDepartmentMembersResponseDto {
        status: "succcess".to_string(),
        members: members_data,
    };

    Ok((StatusCode::OK, Json(response)))
}