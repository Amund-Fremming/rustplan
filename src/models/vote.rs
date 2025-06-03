use super::DayOfWeek;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Vote {
    pub id: i32,
    pub group_id: Uuid,
    pub member_id: i32,
    pub week_number: i32,
    pub day_of_week: DayOfWeek,
}

impl Vote {
    pub fn new(group_id: Uuid, member_id: i32, week_number: i32, day_of_week: DayOfWeek) -> Self {
        Self {
            id: 0,
            group_id,
            member_id,
            week_number,
            day_of_week,
        }
    }
}
