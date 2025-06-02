use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::{Group, Member};

pub async fn create_group(db_pool: &Pool<Postgres>, group: Group) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO "group" (id, name, description, year)
        VALUES ($1, $2, $3, $4);
        "#,
        group.id,
        group.name,
        group.description,
        group.year
    )
    .execute(db_pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_group(db_pool: &Pool<Postgres>, id: Uuid) -> Result<Option<Group>, sqlx::Error> {
    sqlx::query_as!(
        Group,
        r#"
        SELECT id, name,description, year
        FROM "group"
        WHERE id= $1
        "#,
        id
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
