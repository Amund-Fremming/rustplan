use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateMemberRequest {
    pub name: String,
    pub group_id: Uuid,
}
