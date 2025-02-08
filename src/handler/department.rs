use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use validator::Validate;
use crate::AppState;
use crate::db::department_repo::DepartmentRepo;
use crate::dtos::department_dto::{AddDepartmentRequestDto, AddDepartmentResponseDto, GetClubDepartmentsRequestDto, GetClubDepartmentsResponseDto};
use crate::error::HttpError;

pub fn department_handler() -> Router {
    Router::new()
        .route("/add", post(add_department))
        .route("/", get(get_club_departments))
}

pub async fn add_department (
    Extension(app_state) : Extension<Arc<AppState>>,
    Json(body) : Json<AddDepartmentRequestDto>
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    let result = app_state.db_client.insert_department(&body.club_name, &body.department_name)
        .await;

    match result {
        Ok(department) => {
            let response = AddDepartmentResponseDto {
                status: "success".to_string(),
                department
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => Err(HttpError::server_error(err.to_string())),
    }
}

pub async fn get_club_departments(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params) : Query<GetClubDepartmentsRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    params.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state.db_client.find_departments_by_club(&params.club_name).await;
    match result {
        Ok(departments) => {
            let response = GetClubDepartmentsResponseDto {
                status: "success".to_string(),
                departments
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => Err(HttpError::server_error(err.to_string())),
    }
}