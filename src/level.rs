use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::rc::Rc;
use std::path::Path;

use display::Display;
use sdl2;
use player;
use player::Player;

#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
pub enum SquareType {
    EMPTY = 0isize,
    WALL = 1isize,
    BOX = 2isize,
    TARGET = 3isize,
    TARGETVALID = 4isize
}

pub struct Level<'a> {
    columns: isize,
    rows: isize,
    boxsize: isize,
    renderer: Display<'a>,
    textures: HashMap<SquareType, Rc<sdl2::render::Texture>>,
    player_textures: HashMap<player::Orientation, Rc<sdl2::render::Texture>>,
    grid: Vec<Vec<Square>>,
    start_position: (usize, usize)
}

impl<'a> Level<'a> {
    pub fn new(sdl_context: & sdl2::Sdl) -> Level<'a> {
        let width : u32 = 20 * 34;
        let height : u32 = 20 * 34;

        let renderer = Display::new(width, height, sdl_context);

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

    pub fn get_start_position(&self) -> (usize, usize) {
        self.start_position
    }

    fn get_level_content(&mut self) -> Vec<Vec<SquareType>> {
        let level_path = Path::new("./resources/level.txt");
        let buffer = BufReader::new(File::open(&level_path).unwrap());
        let lines: Vec<String> = buffer.lines().map(|x| x.unwrap()).collect();

        let mut grid: Vec<Vec<SquareType>> = Vec::new();
        for line in lines.iter() {
            let mut row: Vec<SquareType> = Vec::new();
            let mut myline = line.clone();
            myline.pop();
            for c in myline.chars() {
                if c == 'P' {
                    self.start_position = (row.len(), grid.len());
                }
                let code: isize = match c.to_digit(4) {
                    Some(d) => d as isize,
                    None => 0isize
                };
                let square_type = match code {
                    0 => SquareType::EMPTY,
                    1 => SquareType::WALL,
                    2 => SquareType::BOX,
                    3 => SquareType::TARGET,
                    4 => SquareType::TARGETVALID,
                    _ => SquareType::EMPTY
                };

                row.push(square_type);
            }
            grid.push(row);
        }
        return grid;
    }

    fn create_grid(content: Vec<Vec<SquareType>>) -> Vec<Vec<Square>> {
        let mut grid: Vec<Vec<Square>> = Vec::new();
        let mut i = 0isize;

        for content_row in content.iter() {
            let mut row: Vec<Square> = Vec::new();
            let mut j = 0isize;
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

    pub fn update_display(&mut self, player: &Player) {
        self.renderer.clear_screen();
        self.renderer.render_grid(&self.grid, self.boxsize as i32, &self.textures);
        let player_texture = match self.player_textures.get(&player.orientation) {
            Some(t) => t,
            None => panic!(format!("error on texture retrieval for player orientation {}", player.orientation.clone() as isize))
        };
        self.renderer.render_player(&**player_texture, player.get_position(), self.boxsize as i32);
        self.renderer.renderer.drawer().present();
    }

    pub fn is_move_allowed(&self, player: &Player, movement: (i8, i8)) -> bool {
        let (x, y) = player.get_position();
        let (dx, dy) = movement;
        let new_x = x as isize + dx as isize;
        let new_y = y as isize + dy as isize;

        match self.grid[new_y as usize][new_x as usize].square_type {
            SquareType::EMPTY => true,
            SquareType::WALL => false,
            SquareType::BOX | SquareType::TARGETVALID => {
                let pos = ((new_x + dx as isize) as usize, (new_y + dy as isize) as usize);
                self.is_free(pos)
            },
            SquareType::TARGET => true
        }
    }
    pub fn is_free(&self, position: (usize, usize)) -> bool {
        let (x, y) = position;
        match self.grid[y][x].square_type {
            SquareType::EMPTY | SquareType::TARGET => true,
            _ => false
        }
    }
    pub fn is_box_present(&self, position: (usize, usize)) -> bool {
        let (x, y) = position;
        match self.grid[y][x].square_type {
            SquareType::BOX | SquareType::TARGETVALID => true,
            _ => false
        }
    }

    pub fn move_box(&mut self, position: (usize, usize), delta: (i8, i8)) {
        let (x, y) = position;
        let (dx, dy) = delta;
        let new_x = (x as isize + dx as isize) as usize;
        let new_y = (y as isize + dy as isize) as usize;
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
    pub x: isize,
    pub y: isize,
    pub square_type: SquareType,
}


pub fn create_square(x: isize, y: isize, square_type: SquareType) -> Square {
    Square{x:x, y:y, square_type: square_type}
}
