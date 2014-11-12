use sdl2;
use sdl2_image;

use std::collections::HashMap;
use std::rc::Rc;

use level;

pub struct Display {
    pub renderer: sdl2::render::Renderer<sdl2::video::Window>
}

impl Display {
    pub fn new(width: int, height: int) -> Display {
        sdl2::init(sdl2::INIT_VIDEO);
        sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

        let window = match sdl2::video::Window::new("Sokoban", sdl2::video::PosCentered, sdl2::video::PosCentered, width, height, sdl2::video::OPENGL) {
            Ok(window) => window,
            Err(err) => fail!(format!("error on window init : {}", err))
        };

        let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
            Ok(renderer) => renderer,
            Err(err) => fail!(format!("failed to create renderer: {}", err))
        };

        Display {
            renderer: renderer
        }
    }

    pub fn get_player_texture(&self) -> sdl2::render::Texture {
        Display::load_texture_from_file("mario_down.gif", &self.renderer)
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

    pub fn clear_screen(&self) {
        let _ = self.renderer.set_draw_color(sdl2::pixels::RGB(255, 255, 255));
        let _ = self.renderer.clear();
    }

    pub fn get_grid_textures(&self) -> HashMap<level::SquareType, Rc<sdl2::render::Texture>> {
        let mut grid_textures: HashMap<level::SquareType, Rc<sdl2::render::Texture>> = HashMap::new();

        let wall_texture = Display::load_texture_from_file("wall.jpg", &self.renderer);
        let box_texture = Display::load_texture_from_file("box.jpg", &self.renderer);
        let target_texture = Display::load_texture_from_file("target.png", &self.renderer);
        let target_valid_texture = Display::load_texture_from_file("box_ok.jpg", &self.renderer);

        grid_textures.insert(level::WALL, Rc::new(wall_texture));
        grid_textures.insert(level::BOX, Rc::new(box_texture));
        grid_textures.insert(level::TARGET, Rc::new(target_texture));
        grid_textures.insert(level::TARGETVALID, Rc::new(target_valid_texture));

        return grid_textures;
    }

    pub fn render_grid(&self, grid: &Vec<Vec<level::Square>>, boxsize: i32) {
        for row in grid.iter() {
            for square in row.iter() {
                let t = square.texture.clone();
                match t {
                    Some(texture) => {
                        let x: i32 = square.x as i32 * boxsize;
                        let y: i32 = square.y as i32 * boxsize;
                        let _ = self.renderer.copy(&*texture, None, Some(sdl2::rect::Rect::new(x, y, boxsize, boxsize)));
                    },
                    None => {}
                }
            }
        }
    }

    pub fn render_player(&self, texture: &sdl2::render::Texture, position: (uint, uint), boxsize: i32) {
        let (x, y) = position;
        let _ = self.renderer.copy(texture, None, Some(sdl2::rect::Rect::new(x as i32 * boxsize, y as i32 * boxsize, boxsize, boxsize)));
    }
}
