use crate::space::SpaceId;

pub struct Space {
    previous: SpaceId,
    nexts: Vec<SpaceId>,
}
