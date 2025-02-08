use std::sync::Arc;

use axum::{middleware, Extension, Router};
use tower_http::trace::TraceLayer;
use tower_http::services::{ServeDir, ServeFile};

use crate::{handler::{auth::auth_handler, users::users_handler}, middleware::auth, AppState};
use crate::handler::admin::admin_handler;
use crate::handler::club::club_handler;
use crate::handler::club_membership_role::club_membership_role_handler;
use crate::handler::department::department_handler;
use crate::handler::membership::membership_handler;
use crate::middleware::auth_admin;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/auth", auth_handler())
        .nest(
            "/users", 
            users_handler()
                .layer(middleware::from_fn(auth))
        )
        .nest(
            "/admin",
            admin_handler()
        )
        .nest(
            "/",
            club_handler()
                .layer(middleware::from_fn(auth_admin))
        )
        .nest(
        "/department",
            department_handler()
                .layer(middleware::from_fn(auth_admin))
        )
        .nest(
            "/club_roles",
            club_membership_role_handler()
        )
        .nest(
            "/membership",
            membership_handler()
        )
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    let pages_route = Router::new()
        .nest_service("/reset-password", ServeFile::new("public/auth/reset-password.html"));

    Router::new()
        .nest("/api", api_route)
        .nest("/", pages_route)
}