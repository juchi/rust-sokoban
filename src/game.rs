
use std::collections::HashMap;
use std::io::BufferedReader;
use std::io::File;

use sdl2;

use display;

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
        let columns : int = 20;
        let rows : int = 20;
        let boxsize : int = 34;

        let renderer = display::init(columns * boxsize, rows * boxsize);

        let grid_textures: HashMap<SquareType, sdl2::render::Texture> = display::get_grid_textures(&renderer);
        let grid_content: Vec<Vec<SquareType>> = get_level_content();
        let grid = create_grid(grid_content, &grid_textures);

        display::clear_screen(&renderer);
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
}

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

fn get_level_content() -> Vec<Vec<SquareType>> {
    let level_path = Path::new("./resources/level.txt");
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

fn create_grid(content: Vec<Vec<SquareType>>, textures: &HashMap<SquareType, sdl2::render::Texture>) -> Vec<Vec<Square>> {
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
