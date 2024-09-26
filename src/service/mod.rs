use crate::{errors::AppError, AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub mod story_service;
pub mod user_service;

pub struct Server {}

pub fn set_app_router(state: AppState) -> Result<Router, AppError> {
    let routers = Router::new()
        .route("/users/register", post(user_service::register))
        .route("/users/login", post(user_service::login))
        .route("/users/:id", get(user_service::get_user))
        .with_state(state);

    Ok(routers)
}
