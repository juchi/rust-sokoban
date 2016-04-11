use sdl2;
use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2_image;

use std::collections::HashMap;
use std::rc::Rc;
use std::path::Path;

use level;
use player;

pub struct Display<'a> {
    pub renderer: Renderer<'a>
}

impl<'a> Display<'a> {
    pub fn new(width: u32, height: u32, sdl_context: & Sdl) -> Display<'a> {
        let _ = sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

        let window = sdl_context.video().unwrap().window("Sokoban", width, height).position_centered().opengl().build().unwrap();
        let renderer = window.renderer().build().unwrap();

        Display {
            renderer: renderer
        }
    }

    pub fn get_player_textures(&self) -> HashMap<player::Orientation, Rc<sdl2::render::Texture>> {
        let mut textures = HashMap::new();
        let up_texture = self.load_texture_from_file("mario_up.gif");
        let down_texture = self.load_texture_from_file("mario_down.gif");
        let left_texture = self.load_texture_from_file("mario_left.gif");
        let right_texture = self.load_texture_from_file("mario_right.gif");

        textures.insert(player::Orientation::UP, Rc::new(up_texture));
        textures.insert(player::Orientation::DOWN, Rc::new(down_texture));
        textures.insert(player::Orientation::LEFT, Rc::new(left_texture));
        textures.insert(player::Orientation::RIGHT, Rc::new(right_texture));

        return textures;
    }

    fn load_texture_from_file(&self, filename: &str) -> sdl2::render::Texture {
        let mut strpath = String::new();
        strpath.push_str("./resources/sprites/");
        strpath.push_str(filename);
        let path = Path::new(&strpath);

        let surface : sdl2::surface::Surface = match sdl2_image::LoadSurface::from_file(&path) {
            Ok(surface) => surface,
            Err(err) => panic!(format!("error on image load {}", err))
        };
        let texture = match self.renderer.create_texture_from_surface(&surface) {
            Ok(texture) => texture,
            Err(sdl2::render::TextureValueError::SdlError(err)) => panic!(format!("error on texture creation {}", err)),
            Err(_) => panic!("Unknown error during texture creation")
        };
        return texture;
    }

    pub fn clear_screen(&mut self) {
        let _ = self.renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        let _ = self.renderer.clear();
    }

    pub fn get_grid_textures(&self) -> HashMap<level::SquareType, Rc<sdl2::render::Texture>> {
        let mut grid_textures: HashMap<level::SquareType, Rc<sdl2::render::Texture>> = HashMap::new();

        let wall_texture = self.load_texture_from_file("wall.jpg");
        let box_texture = self.load_texture_from_file("box.jpg");
        let target_texture = self.load_texture_from_file("target.png");
        let target_valid_texture = self.load_texture_from_file("box_ok.jpg");

        grid_textures.insert(level::SquareType::WALL, Rc::new(wall_texture));
        grid_textures.insert(level::SquareType::BOX, Rc::new(box_texture));
        grid_textures.insert(level::SquareType::TARGET, Rc::new(target_texture));
        grid_textures.insert(level::SquareType::TARGETVALID, Rc::new(target_valid_texture));

        return grid_textures;
    }

    pub fn render_grid(&mut self, grid: &Vec<Vec<level::Square>>, boxsize: u32, textures: & HashMap<level::SquareType, Rc<sdl2::render::Texture>>) {
        for row in grid.iter() {
            for square in row.iter() {
                let t = textures.get(&square.square_type);
                match t {
                    Some(texture) => {
                        let x: i32 = square.x as i32 * boxsize as i32;
                        let y: i32 = square.y as i32 * boxsize as i32;
                        let _ = self.renderer.copy(&**texture, None, Some(sdl2::rect::Rect::new(x, y, boxsize, boxsize)));
                    },
                    None => {}
                }
            }
        }
    }

    pub fn render_player(&mut self, texture: &sdl2::render::Texture, position: (usize, usize), boxsize: u32) {
        let (x, y) = position;
        let _ = self.renderer.copy(texture, None, Some(sdl2::rect::Rect::new(x as i32 * boxsize as i32, y as i32 * boxsize as i32, boxsize, boxsize)));
    }
}
