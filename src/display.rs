use sdl2;
use sdl2::video::WindowPos;
use sdl2_image;

use std::collections::HashMap;
use std::rc::Rc;

use level;
use player;

pub struct Display {
    pub renderer: sdl2::render::Renderer
}

impl Display {
    pub fn new(width: isize, height: isize) -> Display {
        sdl2::init(sdl2::INIT_VIDEO);
        sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

        let window = match sdl2::video::Window::new("Sokoban", WindowPos::PosCentered, WindowPos::PosCentered, width, height, sdl2::video::OPENGL) {
            Ok(window) => window,
            Err(err) => panic!(format!("error on window init : {}", err))
        };

        let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
            Ok(renderer) => renderer,
            Err(err) => panic!(format!("failed to create renderer: {}", err))
        };

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
        let mut path = Path::new("./resources/sprites");
        path.push(filename);

        let surface = match sdl2_image::LoadSurface::from_file(&path) {
            Ok(surface) => surface,
            Err(err) => panic!(format!("error on image load {}", err))
        };
        let texture = match self.renderer.create_texture_from_surface(&surface) {
            Ok(texture) => texture,
            Err(err) => panic!(format!("error on texture creation {}", err))
        };
        return texture;
    }

    pub fn clear_screen(&self) {
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

    pub fn render_grid(&self, grid: &Vec<Vec<level::Square>>, boxsize: i32, textures: & HashMap<level::SquareType, Rc<sdl2::render::Texture>>) {
        for row in grid.iter() {
            for square in row.iter() {
                let t = textures.get(&square.square_type);
                match t {
                    Some(texture) => {
                        let x: i32 = square.x as i32 * boxsize;
                        let y: i32 = square.y as i32 * boxsize;
                        let _ = self.renderer.copy(&**texture, None, Some(sdl2::rect::Rect::new(x, y, boxsize, boxsize)));
                    },
                    None => {}
                }
            }
        }
    }

    pub fn render_player(&self, texture: &sdl2::render::Texture, position: (usize, usize), boxsize: i32) {
        let (x, y) = position;
        let _ = self.renderer.copy(texture, None, Some(sdl2::rect::Rect::new(x as i32 * boxsize, y as i32 * boxsize, boxsize, boxsize)));
    }
}
