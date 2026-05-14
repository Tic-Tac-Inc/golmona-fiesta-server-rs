#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(u8);

impl PlayerId {
    pub fn get(self) -> u8 {
        self.0
    }

    pub fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }
}
