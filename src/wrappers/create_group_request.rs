#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: Option<String>,
    pub year: i32,
}
