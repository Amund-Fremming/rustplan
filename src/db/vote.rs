use crate::models::{DayOfWeek, Vote};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_vote(db_pool: &Pool<Postgres>, vote: Vote) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO vote (id, group_id, member_id, week_number, day_of_week)
        VALUES ($1, $2, $3, $4, $5);
        "#,
        vote.id,
        vote.group_id,
        vote.member_id,
        vote.week_number,
        vote.day_of_week as DayOfWeek
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
