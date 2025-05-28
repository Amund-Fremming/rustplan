use sqlx::{Pool, Postgres};
use tracing::info;

use crate::models::Member;

pub async fn get_member_by_id(
    db_pool: &Pool<Postgres>,
    id: i32,
) -> Result<Option<Member>, sqlx::Error> {
    let member = sqlx::query_as!(
        Member,
        r#"
        SELECT id, group_id, locked_reply
        FROM member
        WHERE id = $1
        "#,
        id,
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(member)
}

pub async fn create_member(db_pool: &Pool<Postgres>, group_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO member (group_id, locked_reply)
        VALUES ($1, $2);
        "#,
        group_id,
        false
    )
    .execute(db_pool)
    .await?;

    info!("Created new member.");
    Ok(())
}
