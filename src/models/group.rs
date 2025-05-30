use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub year: i32,
}

impl Group {
    pub fn new(name: String, description: Option<String>, year: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            year,
        }
    }
}
