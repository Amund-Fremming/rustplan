use sqlx::Type;

#[derive(Type, serde::Deserialize, serde::Serialize, Clone, Debug)]
#[sqlx(type_name = "dayofweek")]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
