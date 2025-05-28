#[derive(Debug, sqlx::FromRow)]
pub struct Member {
    pub id: i32,
    pub group_id: i32,
    pub locked_reply: bool,
}

impl Member {
    pub fn new(group_id: i32, locked_reply: bool) -> Self {
        Self {
            id: 0,
            group_id,
            locked_reply,
        }
    }
}
