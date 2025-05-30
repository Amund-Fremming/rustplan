use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::Member;

pub async fn get_member_by_name(
    db_pool: &Pool<Postgres>,
    name: &String,
) -> Result<Option<Member>, sqlx::Error> {
    sqlx::query_as!(
        Member,
        r#"
        SELECT id, group_id, locked_reply, name
        FROM member
        WHERE name = $1
        "#,
        name,
    )
    .fetch_optional(db_pool)
    .await
}

pub async fn get_members_from_group(
    db_pool: &Pool<Postgres>,
    code: Uuid,
) -> Result<Vec<Member>, sqlx::Error> {
    sqlx::query_as!(
        Member,
        r#"
        SELECT id, group_id, name, locked_reply
        FROM member
        WHERE member.group_id = $1;
        "#,
        code
    )
    .fetch_all(db_pool)
    .await
}

pub async fn create_member(
    db_pool: &Pool<Postgres>,
    name: &String,
    group_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO member (group_id, name, locked_reply)
        VALUES ($1, $2, $3);
        "#,
        group_id,
        name,
        false
    )
    .execute(db_pool)
    .await?;

    Ok(())
}
