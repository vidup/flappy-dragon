use crate::player::Player;
use bracket_lib::prelude::BTerm;
use bracket_lib::prelude::GameState;
use bracket_lib::prelude::VirtualKeyCode;
use bracket_lib::prelude::NAVY;

pub enum GameMode {
    Menu,
    Playing,
    Paused,
    Over,
}

/** State structure & logic to match game engine's state structure */
pub struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
}

fn listen_to_menu_inputs(state: &mut State, ctx: &mut BTerm) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::P => state.restart(),
            VirtualKeyCode::Q => {
                ctx.quitting = true;
            }
            _ => {}
        }
    }
}

impl State {
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
    }

    fn playing(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > crate::FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.apply_gravity();
            self.player.move_forward();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap_wings();
        }

        self.player.render(ctx);
        ctx.print_centered(5, "Press SPACE to flap.");

        if self.player.y >= crate::SCREEN_HEIGHT {
            self.mode = GameMode::Over;
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "Press [P] to play");
        ctx.print_centered(10, "Press [Q] to quit");

        listen_to_menu_inputs(self, ctx)
    }

    fn paused(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Paused");
    }

    fn over(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "You are dead !");
        ctx.print_centered(8, "Press [P] to play");
        ctx.print_centered(10, "Press [Q] to quit");

        listen_to_menu_inputs(self, ctx)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clears the window

        match self.mode {
            GameMode::Menu => {
                self.main_menu(ctx);
            }
            GameMode::Playing => {
                self.playing(ctx);
            }
            GameMode::Paused => {
                self.paused(ctx);
            }
            GameMode::Over => {
                self.over(ctx);
            }
        }
    }
}
