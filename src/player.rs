
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
            orientation: Orientation::DOWN
        }
    }

    pub fn get_position(&self) -> (uint, uint) {
        self.position
    }

    pub fn set_position(&mut self, new_pos: (uint, uint)) {
        self.position = new_pos;
    }

    pub fn update_orientation(&mut self, movement: (i8, i8)) {
        let (dx, dy) = movement;
        if dy == -1 {
            self.orientation = Orientation::UP;
        } else if dy == 1 {
            self.orientation = Orientation::DOWN;
        } else if dx == -1 {
            self.orientation = Orientation::LEFT;
        } else if dx == 1 {
            self.orientation = Orientation::RIGHT;
        }
    }
}
