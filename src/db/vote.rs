use crate::{
    models::{DayOfWeek, Vote},
    wrappers::CreateVoteRequest,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_vote(db_pool: &Pool<Postgres>, vote: Vote) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO vote (group_id, member_id, week_number, day_of_week)
        VALUES ($1, $2, $3, $4);
        "#,
        vote.group_id,
        vote.member_id,
        vote.week_number,
        vote.day_of_week as DayOfWeek
    )
    .execute(db_pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_vote(
    db_pool: &Pool<Postgres>,
    request: &CreateVoteRequest,
) -> Result<Option<Vote>, sqlx::Error> {
    sqlx::query_as!(
        Vote,
        r#"
        SELECT id, group_id, member_id, week_number, day_of_week as "day_of_week: DayOfWeek"
        FROM vote
        WHERE group_id = $1 AND member_id = $2 AND week_number = $3 AND day_of_week = $4
        "#,
        request.group_id,
        request.member_id,
        request.week_number,
        request.day_of_week as DayOfWeek
    )
    .fetch_optional(db_pool)
    .await
}

pub async fn delete_vote(db_pool: &Pool<Postgres>, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM vote
        WHERE id = $1
        "#,
        id
    )
    .execute(db_pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_votes_from_group(
    db_pool: &Pool<Postgres>,
    group_id: Uuid,
) -> Result<Vec<Vote>, sqlx::Error> {
    sqlx::query_as::<_, Vote>(
        r#"
        SELECT id, group_id, member_id, week_number, day_of_week
        FROM vote
        WHERE group_id = $1;
        "#,
    )
    .bind(group_id)
    .fetch_all(db_pool)
    .await
}
