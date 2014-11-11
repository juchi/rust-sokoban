extern crate sdl2;
extern crate sdl2_image;

mod control;
mod display;
mod game;
mod level;
mod player;

fn main() {
    let mut mygame = game::Game::new();
    mygame.start();
}
