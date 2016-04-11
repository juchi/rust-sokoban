use sdl2;

use control;
use level;
use player::Player;

pub struct Game<'a> {
    ctrl: control::Control,
    level: Option<level::Level<'a>>,
    sdl_context: sdl2::Sdl
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            ctrl: control::Control::new(),
            level: None,
            sdl_context: sdl2::init().unwrap()
        }
    }
    pub fn start(&mut self) {
        let mut level = level::Level::new(&self.sdl_context);
        level.init();
        let player = Player::new(level.get_start_position());
        self.level = Some(level);
        self.run(player);
    }

    pub fn run(&mut self, mut player: Player) {
        'main : loop {
            self.ctrl.update(&mut self.sdl_context);
            if self.ctrl.request_quit {
                break;
            }
            match self.level {
                Some(ref mut lvl) => {
                    if self.ctrl.request_move != (0, 0) {
                        if lvl.is_move_allowed(&player, self.ctrl.request_move) {
                            let (x, y) = player.get_position();
                            let (dx, dy) = self.ctrl.request_move;
                            let new_x = x as isize + dx as isize;
                            let new_y = y as isize + dy as isize;
                            let new_pos = (new_x as usize, new_y as usize);
                            if lvl.is_box_present(new_pos) {
                                lvl.move_box(new_pos, (dx, dy));
                            }
                            player.set_position(new_pos);
                        }

                        player.update_orientation(self.ctrl.request_move);
                        self.ctrl.request_move = (0, 0);
                    }
                    lvl.update_display(&player);
                },
                None => {}
            }
        }
    }
}
