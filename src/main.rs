extern crate sdl2;
extern crate sdl2_image;

mod display;
mod game;

fn main() {
    let mygame = game::Game::new();
    mygame.start();
}
