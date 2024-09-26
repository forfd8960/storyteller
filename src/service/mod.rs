use crate::errors::AppError;
use axum::{
    routing::{get, post},
    Router,
};
use serde::Serialize;

pub mod story_service;
pub mod user_service;

#[derive(Debug, Serialize)]
pub enum APIResponse {
    OK,
    User,
    Story,
    Message(String),
}

pub struct Server {}

pub fn set_app_router() -> Result<Router, AppError> {
    let routers = Router::new()
        .route("/users/register", post(user_service::register))
        .route("/users/login", post(user_service::login))
        .route("/users/:id", get(user_service::get_user));

    Ok(routers)
}
