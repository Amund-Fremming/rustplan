use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    db,
    models::{AppState, ServerError, Vote},
    wrappers::CreateVoteRequest,
};

pub async fn create_vote(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateVoteRequest>,
) -> impl IntoResponse {
    let vote = Vote::new(
        request.group_id,
        request.member_id,
        request.week_number,
        request.day_of_week,
    );
    let result = db::create_vote(&state.get_pool(), vote).await?;

    if result == false {
        return Err(ServerError::CriticalError(String::from("Insert failed.")));
    }

    Ok(())
}
