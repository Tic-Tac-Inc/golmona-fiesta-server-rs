use std::collections::HashMap;

use crate::player::{Player, PlayerId};

pub struct Players {
    inner: HashMap<PlayerId, Player>,
    id_sequence: Option<PlayerId>,
}

impl Players {
    fn next_id(&mut self) -> Option<PlayerId> {
        let id = self.id_sequence?;
        self.id_sequence = id.next();
        Some(id)
    }

    pub fn add(&mut self, name: String) -> Option<PlayerId> {
        let id = self.next_id()?;

        self.inner.insert(id, Player::new(name));

        Some(id)
    }

    pub fn get(&self, player_id: &PlayerId) -> Option<&Player> {
        self.inner.get(player_id)
    }

    pub fn get_mut(&mut self, player_id: &PlayerId) -> Option<&mut Player> {
        self.inner.get_mut(player_id)
    }

    pub fn remove(&mut self, player_id: &PlayerId) -> Option<Player> {
        self.inner.remove(player_id)
    }
}
