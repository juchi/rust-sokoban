extern crate sdl2;
extern crate sdl2_image;

use std::io::fs::PathExtensions;
use std::collections::HashMap;

use std::io::BufferedReader;
use std::io::File;

mod game;

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);
    sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

    let columns : int = 20;
    let rows : int = 20;
    let boxsize : int = 34;

    let window = match sdl2::video::Window::new("Sokoban", sdl2::video::PosCentered, sdl2::video::PosCentered, columns * boxsize, rows * boxsize, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => fail!(format!("error on window init : {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    let _ = renderer.set_draw_color(sdl2::pixels::RGB(255, 255, 255));
    let _ = renderer.clear();


    let mygame = game::Game::new();
    mygame.start();

    let mut grid_textures: HashMap<SquareType, sdl2::render::Texture> = HashMap::new();

    let wall_texture = load_texture_from_file("mur.jpg", &renderer);
    let box_texture = load_texture_from_file("caisse.jpg", &renderer);
    let target_texture = load_texture_from_file("objectif.png", &renderer);
    let target_valid_texture = load_texture_from_file("caisse_ok.jpg", &renderer);

    grid_textures.insert(WALL, wall_texture);
    grid_textures.insert(BOX, box_texture);
    grid_textures.insert(TARGET, target_texture);
    grid_textures.insert(TARGETVALID, target_valid_texture);

    let mut grid_content: Vec<Vec<SquareType>> = get_level_content();

/*
    for i in range(0, rows) {
        let mut row = Vec::new();
        for j in range(0, columns) {
            let square_type = match FromPrimitive::from_uint(std::rand::random::<uint>() % 5) {
                Some(t) => t,
                None => EMPTY
            };
            row.push(square_type);
        }
        grid_content.push(row);
    }*/
    let grid = create_grid(grid_content, &grid_textures);
    println!("grid created");
    render_grid(&renderer, &grid, boxsize as i32);

    renderer.present();

    'main : loop {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => break 'main,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        break 'main
                    }
                },
                sdl2::event::NoEvent => break 'event,
                _ => {}
            }
        }
    }
    sdl2::quit();
}
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
pub struct Square<'a> {
    x: int,
    y: int,
    square_type: SquareType,
    texture: Option<&'a sdl2::render::Texture>
}

impl<'a> Square<'a> {
    pub fn set_texture(&mut self, texture: &'a sdl2::render::Texture) -> () {
        self.texture = Some(texture)
    }
}

pub fn load_texture_from_file(filename: &str, renderer: &sdl2::render::Renderer<sdl2::video::Window>) -> sdl2::render::Texture {
    let mut path = Path::new("./target/data/sprites");
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
pub fn get_level_content() -> Vec<Vec<SquareType>> {
    let level_path = Path::new("./target/data/level.txt");
    let mut file = BufferedReader::new(File::open(&level_path));
    let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

    let mut grid: Vec<Vec<SquareType>> = Vec::new();
    for line in lines.iter() {
        let mut row: Vec<SquareType> = Vec::new();
        let mut myline = line.clone();
        myline.pop();
        let slice: &str = myline.as_slice();
        for c in slice.chars() {
            let code: int = match c.to_digit(4) {
                Some(d) => d as int,
                None => 0i
            };
            let square_type = match FromPrimitive::from_int(code) {
                Some(t) => t,
                None => EMPTY
            };
            row.push(square_type);
        }
        grid.push(row);
    }
    return grid;
}

pub fn create_grid(content: Vec<Vec<SquareType>>, textures: &HashMap<SquareType, sdl2::render::Texture>) -> Vec<Vec<Square>> {
    let mut grid: Vec<Vec<Square>> = Vec::new();
    let mut i = 0i;

    for content_row in content.iter() {
        let mut row: Vec<Square> = Vec::new();
        let mut j = 0i;
        for square_type in content_row.iter() {
            let mut tex: Option<&sdl2::render::Texture> = None;
            if *square_type != EMPTY {
                tex = match textures.find(square_type) {
                    Some(texture) => Some(texture),
                    None => fail!(format!("error on texture retrieval for type {}", *square_type as int))
                };
            }
            row.push(create_square(j, i, *square_type, tex));
            j += 1;
        }
        i += 1;
        grid.push(row);
    }
    return grid;
}

pub fn create_square(x: int, y: int, square_type: SquareType, texture: Option<&sdl2::render::Texture>) -> Square {
    Square{x:x, y:y, square_type: square_type, texture:texture}
}

pub fn render_grid(renderer: &sdl2::render::Renderer<sdl2::video::Window>, grid: &Vec<Vec<Square>>, boxsize: i32) {
    for row in grid.iter() {
        for square in row.iter() {
            match square.texture {
                Some(texture) => {
                    let x: i32 = square.x as i32 * boxsize;
                    let y: i32 = square.y as i32 * boxsize;
                    let _ = renderer.copy(texture, None, Some(sdl2::rect::Rect::new(x, y, boxsize, boxsize)));
                },
                None => {}
            }
        }
    }
}
