pub struct GroupDto {
    group_name: String,
    weeks: Vec<Week>,
}

struct Week {
    week_number: i32,
    rows: Vec<Row>,
}

struct Row {
    member_name: String,
    days: Vec<bool>,
}
