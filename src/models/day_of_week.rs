use serde::{Deserialize, Deserializer};
use sqlx::Type;

#[derive(Type, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash, Copy)]
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

impl<'de> Deserialize<'de> for DayOfWeek {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = usize::deserialize(deserializer)?;
        DayOfWeek::try_from(value)
            .map_err(|_| serde::de::Error::custom(format!("Invalid day number: {}", value)))
    }
}

impl TryFrom<usize> for DayOfWeek {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DayOfWeek::Monday),
            1 => Ok(DayOfWeek::Tuesday),
            2 => Ok(DayOfWeek::Wednesday),
            3 => Ok(DayOfWeek::Thursday),
            4 => Ok(DayOfWeek::Friday),
            5 => Ok(DayOfWeek::Saturday),
            6 => Ok(DayOfWeek::Sunday),
            _ => Err(()),
        }
    }
}
