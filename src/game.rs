extern crate sdl2;

use std::io::fs::PathExtensions;


pub struct Game;

impl Game {
    pub fn new() -> Game {
        Game
    }
    pub fn start(&self) {
        println!("start game");
    }
}
