use sdl2;

use control;
use level;

pub struct Game {
    ctrl: control::Control,
    level: Option<level::Level>
}

impl Game {
    pub fn new() -> Game {
        Game {
            ctrl: control::Control::new(),
            level: None
        }
    }
    pub fn start(&mut self) {
        let mut level = level::Level::new();
        level.init();
        self.level = Some(level);
        self.run();
        sdl2::quit();
    }

    pub fn run(&mut self) {
        'main : loop {
            self.ctrl.update();
            if self.ctrl.request_quit {
                break;
            }
            match self.level {
                Some(ref lvl) => lvl.update_display(),
                None => {}
            }
        }
    }
}
