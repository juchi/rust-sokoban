use sdl2;
use sdl2_image;

use std::collections::HashMap;

use game;

pub fn init(width: int, height: int) -> sdl2::render::Renderer<sdl2::video::Window> {
    let window = match sdl2::video::Window::new("Sokoban", sdl2::video::PosCentered, sdl2::video::PosCentered, width, height, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => fail!(format!("error on window init : {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    return renderer;
}

fn load_texture_from_file(filename: &str, renderer: &sdl2::render::Renderer<sdl2::video::Window>) -> sdl2::render::Texture {
    let mut path = Path::new("./resources/sprites");
    path.push(filename);

    let surface = match sdl2_image::LoadSurface::from_file(&path) {
        Ok(surface) => surface,
        Err(err) => fail!(format!("error on image load {}", err))
    };
    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(texture) => texture,
        Err(err) => fail!(format!("error on texture creation {}", err))
    };
    return texture;
}

pub fn get_grid_textures(renderer: &sdl2::render::Renderer<sdl2::video::Window>) -> HashMap<game::SquareType, sdl2::render::Texture> {
    let mut grid_textures: HashMap<game::SquareType, sdl2::render::Texture> = HashMap::new();

    let wall_texture = load_texture_from_file("wall.jpg", renderer);
    let box_texture = load_texture_from_file("box.jpg", renderer);
    let target_texture = load_texture_from_file("target.png", renderer);
    let target_valid_texture = load_texture_from_file("box_ok.jpg", renderer);

    grid_textures.insert(game::WALL, wall_texture);
    grid_textures.insert(game::BOX, box_texture);
    grid_textures.insert(game::TARGET, target_texture);
    grid_textures.insert(game::TARGETVALID, target_valid_texture);

    return grid_textures;
}
