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

    pub fn run(&mut self, mut player: Player) {
        'main : loop {
            self.ctrl.update();
            if self.ctrl.request_quit {
                break;
            }
            if self.ctrl.request_move != (0, 0) {
                let (x, y) = player.get_position();
                let (dx, dy) = self.ctrl.request_move;
                let new_x = x as int + dx as int;
                let new_y = y as int + dy as int;
                player.update_orientation(self.ctrl.request_move);
                player.set_position ((new_x as uint, new_y as uint));
                self.ctrl.request_move = (0, 0);
            }
            match self.level {
                Some(ref lvl) => lvl.update_display(&player),
                None => {}
            }
        }
    }
}
