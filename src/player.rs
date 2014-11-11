
pub struct Player {
    position: (uint, uint)
}

impl Player {
    pub fn new(pos: (uint, uint)) -> Player {
        Player {
            position: pos
        }
    }

    pub fn get_position(&self) -> (uint, uint) {
        self.position
    }
}
