use std::sync::Arc;
use axum::{middleware, Extension, Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use validator::{Validate, ValidationError};
use crate::AppState;
use crate::db::club_membership_role_repo::ClubMembershipRoleRepo;
use crate::dtos::club_membership_role_dto::{AddClubMembershipRoleRequestDto, AddClubMembershipRoleResponseDto, GetAllClubMembershipRolesResponse};
use crate::error::HttpError;
use crate::middleware::auth_admin;

pub fn club_membership_role_handler() -> Router {
    Router::new()
        .route("/add", post(add_club_membership_role).layer(middleware::from_fn(auth_admin)))
        .route("/all", get(get_club_membership_roles))
}

pub async fn add_club_membership_role(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body) : Json<AddClubMembershipRoleRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    body.validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let result = app_state.db_client.add_membership_role(&body.role_name)
        .await;

    match result {
        Ok(role) => {
            let response = AddClubMembershipRoleResponseDto {
                status: "success".to_string(),
                membership_role: role
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => Err(HttpError::server_error(err.to_string()))?
    }
}

pub async fn get_club_membership_roles (
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {

    let result = app_state.db_client.get_all_membership_roles().await;
    match result {
        Ok (roles) => {
            let response = GetAllClubMembershipRolesResponse {
                status: "seccuss".to_string(),
                membership_roles : roles
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => Err(HttpError::server_error(err.to_string()))?
    }
}