use uuid::Uuid;

pub struct CreateMemberRequest {
    pub name: String,
    pub group_id: Uuid,
}
