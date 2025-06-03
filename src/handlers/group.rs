use std::{collections::HashSet, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    db,
    models::{AppState, DayOfWeek, Group, Member, ServerError},
    wrappers::{CreateGroupRequest, CreateMemberRequest, GroupDto, Row, Week},
};

pub async fn get_group_with_relations(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ServerError> {
    let group_res = db::get_group(state.get_pool(), id).await?;
    if let None = group_res {
        return Err(ServerError::HandlerError(
            StatusCode::NOT_FOUND,
            format!("Group with id {} does not exist.", id),
        ));
    }

    let group = db::get_group(state.get_pool(), id)
        .await?
        .ok_or(ServerError::HandlerError(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Failed to fetch data"),
        ))?;

    let members = db::get_members_from_group(state.get_pool(), id).await?;
    let votes = db::get_votes_from_group(state.get_pool(), id).await?;
    let vote_map: HashSet<(i32, i32, DayOfWeek)> = votes
        .iter()
        .map(|vote| (vote.member_id, vote.week_number, vote.day_of_week))
        .collect();

    let mut group_dto = GroupDto::new(group.name);

    for week_nr in 1..=52 {
        let mut week = Week::new(week_nr);
        for member in &members {
            let mut row = Row::new((*member.name).to_string());
            let member_id = &member.id;
            for day_nr in 0..7 {
                let key = (*member_id, week_nr, DayOfWeek::try_from(day_nr).unwrap());
                let has_voted = vote_map.contains(&key);
                row.set_vote(day_nr, has_voted);
            }
            week.add_row(row);
        }

        group_dto.add_week(week);
    }

    Ok(Json(group_dto))
}

pub async fn create_group(
    State(state): State<Arc<AppState>>,
    Json(group_params): Json<CreateGroupRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let group = Group::new(
        group_params.name,
        group_params.description,
        group_params.year,
    );
    let result = db::create_group(state.get_pool(), group.clone()).await?;

    if result == false {
        return Err(ServerError::CriticalError(String::from(
            "Failed to insert data.",
        )));
    }

    Ok((StatusCode::CREATED, Json(group.id)).into_response())
}

pub async fn add_member_to_group(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateMemberRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let member = Member::new(request.group_id, request.name.clone());
    let group_members = db::get_members_from_group(state.get_pool(), request.group_id).await?;

    if group_members
        .iter()
        .all(|member| member.name != request.name)
    {
        db::create_member(&state.get_pool(), member).await?;
    }

    Ok(StatusCode::OK)
}

pub async fn remove_member_from_group(
    State(state): State<Arc<AppState>>,
    Path((group_id, member_id)): Path<(Uuid, i32)>,
) -> Result<impl IntoResponse, ServerError> {
    let group = db::get_group(state.get_pool(), group_id).await?;
    if let None = group {
        return Ok(StatusCode::NOT_FOUND);
    }

    let member = db::get_member_by_id(state.get_pool(), member_id).await?;
    if let None = member {
        return Ok(StatusCode::NOT_FOUND);
    }

    let result = db::remove_member_from_group(state.get_pool(), member_id).await?;
    match result {
        true => Ok(StatusCode::OK),
        false => Err(ServerError::CriticalError(String::from(
            "Failed to delete to database.",
        ))),
    }
}
