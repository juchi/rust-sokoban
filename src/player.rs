
#[deriving(Hash)]
#[deriving(Eq)]
#[deriving(PartialEq)]
pub enum Orientation {
    UP = 0i,
    DOWN = 1i,
    LEFT = 2i,
    RIGHT = 3i
}
pub struct Player {
    position: (uint, uint),
    pub orientation: Orientation
}

impl Player {
    pub fn new(pos: (uint, uint)) -> Player {
        Player {
            position: pos,
            orientation: DOWN
        }
    }

    pub fn get_position(&self) -> (uint, uint) {
        self.position
    }
}
