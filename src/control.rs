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
    pub fn update(&mut self) {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::Event::Quit(_) => self.request_quit = true,
                sdl2::event::Event::KeyDown(_, _, key, _, _, _) => {
                    match key {
                        sdl2::keycode::KeyCode::Escape => self.request_quit = true,
                        sdl2::keycode::KeyCode::Up => self.request_move = (0, -1),
                        sdl2::keycode::KeyCode::Down => self.request_move = (0, 1),
                        sdl2::keycode::KeyCode::Left => self.request_move = (-1, 0),
                        sdl2::keycode::KeyCode::Right => self.request_move = (1, 0),
                        _ => ()
                    }
                },
                sdl2::event::Event::None => break 'event,
                _ => {}
            }
        }
    }
}
