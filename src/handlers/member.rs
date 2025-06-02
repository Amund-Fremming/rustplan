use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

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
