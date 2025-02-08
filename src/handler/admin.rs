use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum_extra::extract::cookie::Cookie;
use validator::Validate;
use crate::AppState;
use crate::db::admin_repo::AdminRepo;
use crate::dtos::admin_dto::{LoginAdminRequestDto, LoginAdminResponseDto, RegisterAdminDtos, RegisterAdminResponseDto};
use crate::error::{ErrorMessage, HttpError};
use crate::utils::{password, token};

pub fn admin_handler() -> Router {
    Router::new()
        .route("/register", post(register_admin))
        .route("/login", post(login_admin))
}

pub async fn register_admin (
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RegisterAdminDtos>
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let hash_password = password::hash(&body.password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = app_state.db_client.insert_admin(&body.email, &body.name, &hash_password)
        .await;
    match result {
        Ok(admin) => {
            let response = RegisterAdminResponseDto {
                status: "success".to_string(),
                admin: admin
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::NameExist.to_string(),
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string()))
    }
}

pub async fn login_admin (
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body) : Json<LoginAdminRequestDto>
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
    .map_err(|err| HttpError::bad_request(err.to_string()))?;

    // let hash_password = password::hash(&body.password)
    //     .map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = app_state.db_client.get_admin(None, Some(&body.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let admin = result.ok_or(HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    let password_matched = password::compare(&body.password, &admin.password)
        .map_err(|_| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    if password_matched {

        let token = token::create_token(
            &admin.id.to_string(),
            &app_state.env.jwt_secret.as_bytes(),
            app_state.env.jwt_maxage
        )
            .map_err(|e| HttpError::server_error(e.to_string()))?;

        let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);
        let cookie = Cookie::build(("token", token.clone()))
            .path("/")
            .max_age(cookie_duration)
            .http_only(true)
            .build();

        let response = Json(LoginAdminResponseDto{
            status: "success".to_string(),
            admin: admin,
            token: token
        });

        let mut headers = HeaderMap::new();

        headers.append(
            header::SET_COOKIE,
            cookie.to_string().parse().unwrap(),
        );
        let mut response = response.into_response();
        response.headers_mut().extend(headers);

        Ok(response)
    }
    else {
        Err(HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))
    }


}