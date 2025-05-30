use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    db,
    models::{AppState, Group, ServerError},
    wrappers::{CreateGameRequest, GroupWithRelations},
};

pub async fn get_group_with_relations(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let group_res = db::get_group(&state.get_pool(), id).await?;
    if let None = group_res {
        return Err(ServerError::HandlerError(
            StatusCode::NOT_FOUND,
            format!("Group with id {} does not exist.", id),
        ));
    }

    let members = db::get_members_from_group(&state.get_pool(), id).await?;
    let votes = db::get_votes_from_group(&state.get_pool(), id).await?;
    let dto = GroupWithRelations::new(group_res.unwrap(), members, votes);

    Ok(Json(dto))
}

pub async fn create_group(
    State(state): State<Arc<AppState>>,
    Json(game_params): Json<CreateGameRequest>,
) -> impl IntoResponse {
    let group = Group::new(game_params.name, game_params.description, game_params.year);
    let result = db::create_group(&state.get_pool(), group).await?;

    if result == false {
        return Err(ServerError::CriticalError(String::from(
            "Failed to insert data.",
        )));
    }

    Ok(())
}
