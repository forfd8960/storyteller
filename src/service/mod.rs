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
        .route(
            "/stories/:id",
            get(story_service::get_story)
                .delete(story_service::delete_story)
                .put(story_service::update_story),
        )
        .route("/stories/list", get(story_service::list_stories))
        .route("/stories", post(story_service::create_story))
        .with_state(state);

    Ok(routers)
}
