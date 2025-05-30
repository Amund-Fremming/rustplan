use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct Member {
    pub id: i32,
    pub group_id: Uuid,
    pub name: String,
    pub locked_reply: bool,
}

impl Member {
    pub fn new(group_id: Uuid, name: String) -> Self {
        Self {
            id: 0,
            group_id,
            name,
            locked_reply: false,
        }
    }
}
