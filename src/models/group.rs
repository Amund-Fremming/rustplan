use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub year: i32,
}

impl Group {
    pub fn new(name: String, desc_option: Option<String>, year: i32) -> Self {
        let description = match desc_option {
            Some(string) => string,
            None => String::new(),
        };

        Self {
            id: Uuid::new_v4(),
            name,
            description,
            year,
        }
    }
}
