use sdl2;

pub struct Control {
    pub request_quit: bool
}
impl Control {
    pub fn new() -> Control {
        Control {
            request_quit: false
        }
    }
    pub fn update(&mut self) {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => self.request_quit = true,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        self.request_quit = true;
                    }
                },
                sdl2::event::NoEvent => break 'event,
                _ => {}
            }
        }
    }
}
