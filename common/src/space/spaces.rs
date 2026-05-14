use std::collections::HashMap;

use crate::space::{Space, SpaceId};

pub struct Spaces(HashMap<SpaceId, Space>);
