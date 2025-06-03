use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use tracing::debug;

use crate::{
    db,
    models::{AppState, ServerError, Vote},
    wrappers::CreateVoteRequest,
};

pub async fn toggle_vote(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateVoteRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let result = db::get_vote(state.get_pool(), &request).await?;
    if let None = result {
        let vote = Vote::new(
            request.group_id,
            request.member_id,
            request.week_number,
            request.day_of_week,
        );

        debug!("Creating user");

        db::create_vote(state.get_pool(), vote).await?;
        return Ok(StatusCode::CREATED);
    }

    debug!("Deleting user");
    let vote = result.unwrap();
    db::delete_vote(state.get_pool(), vote.id).await?;

    Ok(StatusCode::OK)
}
