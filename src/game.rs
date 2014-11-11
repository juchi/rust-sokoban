use sdl2;

use control;
use level;
use player::Player;

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
        let player = Player::new(level.get_start_position());
        self.level = Some(level);
        self.run(player);
        sdl2::quit();
    }

    pub fn run(&mut self, player: Player) {
        'main : loop {
            self.ctrl.update();
            if self.ctrl.request_quit {
                break;
            }
            match self.level {
                Some(ref lvl) => lvl.update_display(&player),
                None => {}
            }
        }
    }
}
