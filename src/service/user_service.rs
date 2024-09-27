use crate::{errors::AppError, models::User, AppState};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LoginResponse {
    user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RegisterResponse {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GetUserResponse(pub User);

pub(crate) async fn register(
    State(state): State<AppState>,
    Json(data): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("register request: {:?}", data);

    Ok(Json(RegisterResponse {
        user: User::default(),
    }))
}
pub(crate) async fn login(
    State(state): State<AppState>,
    Json(data): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("login request: {:?}", data);

    Ok(Json(LoginResponse {
        user: User::default(),
    }))
}

pub(crate) async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!("user_id: {}", user_id);

    Ok(Json(GetUserResponse(User::default())))
}
