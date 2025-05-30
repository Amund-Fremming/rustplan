use crate::models::{Group, Member, Vote};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GroupWithRelations {
    group: Group,
    members: Vec<Member>,
    votes: Vec<Vote>,
}

impl GroupWithRelations {
    pub fn new(group: Group, members: Vec<Member>, votes: Vec<Vote>) -> Self {
        Self {
            group,
            members,
            votes,
        }
    }
}
