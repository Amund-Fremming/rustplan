use uuid::Uuid;

pub struct Group {
    id: i128,
    code: Uuid,
    name: String,
    description: Option<String>,
    year: u8,
}

impl Group {
    //
}
