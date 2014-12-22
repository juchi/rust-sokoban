use std::collections::HashMap;
use std::io::BufferedReader;
use std::io::File;
use std::rc::Rc;

use display::Display;
use sdl2;
use player;
use player::Player;

#[deriving(Clone)]
#[deriving(FromPrimitive)]
#[deriving(Hash)]
#[deriving(Eq)]
#[deriving(PartialEq)]
pub enum SquareType {
    EMPTY = 0i,
    WALL = 1i,
    BOX = 2i,
    TARGET = 3i,
    TARGETVALID = 4i
}

pub struct Level {
    columns: int,
    rows: int,
    boxsize: int,
    renderer: Display,
    textures: HashMap<SquareType, Rc<sdl2::render::Texture>>,
    player_textures: HashMap<player::Orientation, Rc<sdl2::render::Texture>>,
    grid: Vec<Vec<Square>>,
    start_position: (uint, uint)
}

impl Level {
    pub fn new() -> Level {
        let width = 20 * 34;
        let height = 20 * 34;
        let renderer = Display::new(width, height);

        Level {
            columns: 20,
            rows: 20,
            boxsize: 34,
            renderer: renderer,
            textures: HashMap::new(),
            player_textures: HashMap::new(),
            grid: Vec::new(),
            start_position: (0, 0)
        }
    }

    pub fn init(&mut self) {
        self.textures = self.renderer.get_grid_textures();
        self.player_textures = self.renderer.get_player_textures();
        let grid_content: Vec<Vec<SquareType>> = self.get_level_content();
        self.grid = Level::create_grid(grid_content);
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
                    None => SquareType::EMPTY
                };
                row.push(square_type);
            }
            grid.push(row);
        }
        return grid;
    }

    fn create_grid(content: Vec<Vec<SquareType>>) -> Vec<Vec<Square>> {
        let mut grid: Vec<Vec<Square>> = Vec::new();
        let mut i = 0i;

        for content_row in content.iter() {
            let mut row: Vec<Square> = Vec::new();
            let mut j = 0i;
            for square_type in content_row.iter() {
                let s = create_square(j, i, square_type.clone());
                row.push(s);
                j += 1;
            }
            i += 1;
            grid.push(row);
        }
        return grid;
    }

    pub fn update_display(&self, player: &Player) {
        self.renderer.clear_screen();
        self.renderer.render_grid(&self.grid, self.boxsize as i32, &self.textures);
        let player_texture = match self.player_textures.get(&player.orientation) {
            Some(t) => t,
            None => panic!(format!("error on texture retrieval for player orientation {}", player.orientation.clone() as int))
        };
        self.renderer.render_player(&**player_texture, player.get_position(), self.boxsize as i32);
        self.renderer.renderer.present();
    }

    pub fn is_move_allowed(&self, player: &Player, movement: (i8, i8)) -> bool {
        let (x, y) = player.get_position();
        let (dx, dy) = movement;
        let new_x = x as int + dx as int;
        let new_y = y as int + dy as int;

        match self.grid[new_y as uint][new_x as uint].square_type {
            SquareType::EMPTY => true,
            SquareType::WALL => false,
            SquareType::BOX | SquareType::TARGETVALID => {
                let pos = ((new_x + dx as int) as uint, (new_y + dy as int) as uint);
                self.is_free(pos)
            },
            SquareType::TARGET => true
        }
    }
    pub fn is_free(&self, position: (uint, uint)) -> bool {
        let (x, y) = position;
        match self.grid[y][x].square_type {
            SquareType::EMPTY | SquareType::TARGET => true,
            _ => false
        }
    }
    pub fn is_box_present(&self, position: (uint, uint)) -> bool {
        let (x, y) = position;
        match self.grid[y][x].square_type {
            SquareType::BOX | SquareType::TARGETVALID => true,
            _ => false
        }
    }

    pub fn move_box(&mut self, position: (uint, uint), delta: (i8, i8)) {
        let (x, y) = position;
        let (dx, dy) = delta;
        let new_x = (x as int + dx as int) as uint;
        let new_y = (y as int + dy as int) as uint;
        match self.grid[y][x].square_type {
            SquareType::BOX => self.grid[y][x].square_type = SquareType::EMPTY,
            SquareType::TARGETVALID => self.grid[y][x].square_type = SquareType::TARGET,
            _ => ()
        };

        match self.grid[new_y][new_x].square_type {
            SquareType::EMPTY => self.grid[new_y][new_x].square_type = SquareType::BOX,
            SquareType::TARGET => self.grid[new_y][new_x].square_type = SquareType::TARGETVALID,
            _ => ()
        };
    }
}

pub struct Square {
    pub x: int,
    pub y: int,
    pub square_type: SquareType,
}


pub fn create_square(x: int, y: int, square_type: SquareType) -> Square {
    Square{x:x, y:y, square_type: square_type}
}
