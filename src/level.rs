use std::collections::HashMap;
use std::io::BufferedReader;
use std::io::File;
use std::rc::Rc;

use display;
use sdl2;
use player::Player;

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

pub struct Level {
    columns: int,
    rows: int,
    boxsize: int,
    renderer: sdl2::render::Renderer<sdl2::video::Window>,
    textures: HashMap<SquareType, Rc<sdl2::render::Texture>>,
    player_texture: sdl2::render::Texture,
    grid: Vec<Vec<Square>>,
    start_position: (uint, uint)
}

impl Level {
    pub fn new() -> Level {
        let width = 20 * 34;
        let height = 20 * 34;
        let renderer = display::init(width, height);
        let p_tex = display::get_player_texture(&renderer);

        Level {
            columns: 20,
            rows: 20,
            boxsize: 34,
            renderer: renderer,
            textures: HashMap::new(),
            player_texture: p_tex,
            grid: Vec::new(),
            start_position: (0, 0)
        }
    }

    pub fn init(&mut self) {
        self.textures = display::get_grid_textures(&self.renderer);
        let grid_content: Vec<Vec<SquareType>> = self.get_level_content();
        let grid = Level::create_grid(grid_content, &self.textures);
        self.grid = grid;
    }

    pub fn get_start_position(&self) -> (uint, uint) {
        self.start_position
    }

    fn get_level_content(&mut self) -> Vec<Vec<SquareType>> {
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
                if c == 'P' {
                    self.start_position = (row.len(), grid.len());
                }
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

    fn create_grid(content: Vec<Vec<SquareType>>, textures: & HashMap<SquareType, Rc<sdl2::render::Texture>>) -> Vec<Vec<Square>> {
        let mut grid: Vec<Vec<Square>> = Vec::new();
        let mut i = 0i;

        for content_row in content.iter() {
            let mut row: Vec<Square> = Vec::new();
            let mut j = 0i;
            for square_type in content_row.iter() {
                let mut tex: Option<Rc<sdl2::render::Texture>> = None;
                if *square_type != EMPTY {
                    tex = match textures.find(square_type) {
                        Some(t) => {let tcopy = t.clone(); Some(tcopy)},
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

    pub fn update_display(&self, player: &Player) {
        display::clear_screen(&self.renderer);
        display::render_grid(&self.renderer, &self.grid, self.boxsize as i32);
        display::render_player(&self.renderer, &self.player_texture, player.get_position(), self.boxsize as i32);
        self.renderer.present();
    }
}

pub struct Square {
    pub x: int,
    pub y: int,
    pub square_type: SquareType,
    pub texture: Option<Rc<sdl2::render::Texture>>
}


pub fn create_square(x: int, y: int, square_type: SquareType, texture: Option<Rc<sdl2::render::Texture>>) -> Square {
    Square{x:x, y:y, square_type: square_type, texture:texture}
}
