use crate::space::SpaceId;

pub struct Player {
    name: String,
    crnt_space: Option<SpaceId>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            crnt_space: None,
        }
    }
}
