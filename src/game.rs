extern crate sdl2;

use std::io::fs::PathExtensions;


#[deriving(FromPrimitive)]
#[deriving(Hash)]
pub enum SquareType {
    EMPTY = 0i,
    WALL = 1i,
    BOX = 2i,
    TARGET = 3i,
    TARGETVALID = 4i
}

impl PartialEq for SquareType {
    fn eq(&self, other: &SquareType) -> bool {
        *self as int == *other as int
    }
}
impl Eq for SquareType {}

pub struct Game;

impl Game {
    pub fn new() -> Game {
        Game
    }
    pub fn start(&self) {
        println!("start game");
    }
}
