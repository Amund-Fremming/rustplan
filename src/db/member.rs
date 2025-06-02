use sqlx::{Pool, Postgres};

use crate::models::Member;

pub async fn get_member_by_id(
    db_pool: &Pool<Postgres>,
    id: i32,
) -> Result<Option<Member>, sqlx::Error> {
    sqlx::query_as!(
        Member,
        r#"
        SELECT id, group_id, locked_reply, name
        FROM member
        WHERE id = $1;
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await
}

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

pub async fn create_member(db_pool: &Pool<Postgres>, member: Member) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO member (group_id, name, locked_reply)
        VALUES ($1, $2, $3);
        "#,
        member.group_id,
        member.name,
        false
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

pub async fn remove_member_from_group(
    db_pool: &Pool<Postgres>,
    member_id: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM member
        WHERE id = $1;
        "#,
        member_id
    )
    .execute(db_pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
