#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GroupDto {
    group_name: String,
    weeks: Vec<Week>,
}

impl GroupDto {
    pub fn new(group_name: String) -> Self {
        Self {
            group_name,
            weeks: Vec::new(),
        }
    }

    pub fn add_week(&mut self, week: Week) {
        self.weeks.push(week);
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Week {
    week_number: i32,
    rows: Vec<Row>,
}

impl Week {
    pub fn new(week_number: i32) -> Self {
        Self {
            week_number,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Row {
    member_name: String,
    days: Vec<bool>,
}

impl Row {
    pub fn new(member_name: String) -> Self {
        Self {
            member_name,
            days: vec![false, false, false, false, false, false, false],
        }
    }

    pub fn set_vote(&mut self, index: usize, value: bool) {
        self.days[index] = value;
    }
}
