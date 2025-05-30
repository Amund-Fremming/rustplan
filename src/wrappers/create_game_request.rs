#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateGameRequest {
    pub name: String,
    pub description: Option<String>,
    pub year: i32,
}
