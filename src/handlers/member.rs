use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    db::{self},
    models::{AppState, ServerError},
};

pub async fn get_member(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let result = db::get_member_by_name(state.get_pool(), &name).await?;

    match result {
        Some(member) => Ok(Json(member)),
        None => Err(ServerError::HandlerError(
            StatusCode::NOT_FOUND,
            format!("User with name {} does not exist.", name),
        )),
    }
}

pub async fn create_member(
    State(state): State<Arc<AppState>>,
    Path((name, group_id)): Path<(String, Uuid)>,
) -> impl IntoResponse {
    match db::create_member(state.get_pool(), &name, group_id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
