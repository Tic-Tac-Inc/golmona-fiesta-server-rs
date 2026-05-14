use crate::{player::Players, space::Spaces};

pub struct World {
    players: Players, // joueurs
    spaces: Spaces,   // cases
                      // minijeu (Option<Minijeu>)
                      // context => focus de la camera
}
