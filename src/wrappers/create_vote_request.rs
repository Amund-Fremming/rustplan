use uuid::Uuid;

use crate::models::DayOfWeek;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateVoteRequest {
    pub group_id: Uuid,
    pub member_id: i32,
    pub week_number: i32,
    pub day_of_week: DayOfWeek,
}
