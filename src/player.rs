
#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
pub enum Orientation {
    UP = 0is,
    DOWN = 1is,
    LEFT = 2is,
    RIGHT = 3is
}
pub struct Player {
    position: (usize, usize),
    pub orientation: Orientation
}

impl Player {
    pub fn new(pos: (usize, usize)) -> Player {
        Player {
            position: pos,
            orientation: Orientation::DOWN
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    pub fn set_position(&mut self, new_pos: (usize, usize)) {
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
