use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    db::{self},
    models::AppState,
};

pub async fn get_member(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::get_member_by_id(state.get_pool(), id).await {
        Ok(Some(_)) => StatusCode::OK,
        Ok(None) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn create_member(
    State(state): State<Arc<AppState>>,
    Path(group_id): Path<i32>,
) -> impl IntoResponse {
    match db::create_member(state.get_pool(), group_id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
