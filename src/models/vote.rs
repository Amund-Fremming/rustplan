use super::DayOfWeek;

pub struct Vote {
    id: i128,
    user_id: i128,
    group_id: i128,
    week_number: u8,
    day: DayOfWeek,
}
