use sdl2;

pub struct Control {
    pub request_quit: bool,
    pub request_move: (i8, i8)
}
impl Control {
    pub fn new() -> Control {
        Control {
            request_quit: false,
            request_move: (0, 0)
        }
    }
    pub fn update(&mut self, sdl_context: &mut sdl2::Sdl) {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => self.request_quit = true,
                sdl2::event::Event::KeyDown { keycode: Option::Some(val), .. }  => {
                    match val {
                        sdl2::keyboard::Keycode::Escape => self.request_quit = true,
                        sdl2::keyboard::Keycode::Up => self.request_move = (0, -1),
                        sdl2::keyboard::Keycode::Down => self.request_move = (0, 1),
                        sdl2::keyboard::Keycode::Left => self.request_move = (-1, 0),
                        sdl2::keyboard::Keycode::Right => self.request_move = (1, 0),
                        _ => ()
                    }
                },
                _ => {}
            }
        }
    }
}
