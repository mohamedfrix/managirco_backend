use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use validator::Validate;
use crate::AppState;
use crate::db::club_repo::ClubRepo;
use crate::error::HttpError;
use crate::dtos::club_dto::{AddClubDto, ClubResponseDto, GetAllClubsResponseDto};

pub fn club_handler() -> Router {
    Router::new()
        .route("/club", post(add_club))
        .route("/club/all", get(get_all_clubs))
}

pub async fn add_club(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<AddClubDto>
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    let result = app_state.db_client.insert_club(&body.name, &body.school, &body.email)
    .await;

    match result {
        Ok(club) => {
            let response_data = ClubResponseDto {
                status: "success".to_string(),
                club: club
            };

            Ok((StatusCode::CREATED, Json(response_data)))
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn get_all_clubs(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
    let result = app_state.db_client.get_all_clubs().await;

    match result {
        Ok(clubs) => {
            let response_data = GetAllClubsResponseDto{
                status: "success".to_string(),
                clubs
            };
            Ok((StatusCode::CREATED, Json(response_data)))
        }
        Err(e) => Err(HttpError::server_error(e.to_string()))
    }
}