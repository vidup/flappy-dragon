use bracket_lib::prelude::*;

/** GAME CONSTANTS */
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

/** MODES */
enum GameMode {
    Menu,
    Playing,
    Paused,
    Over,
}

/** Player structure */
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    /** This function renders the player as a yellow "@" character */
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn apply_gravity(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn move_forward(&mut self) {
        self.x += 1;
    }

    fn flap_wings(&mut self) {
        self.velocity = -2.0;
    }
}

/** State structure & logic to match game engine's state structure */
struct State {
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
    fn new() -> Self {
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

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.apply_gravity();
            self.player.move_forward();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap_wings();
        }

        self.player.render(ctx);
        ctx.print_centered(5, "Press SPACE to flap.");

        if self.player.y >= SCREEN_HEIGHT {
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

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Bracket Terminal")
        .build()?;

    main_loop(context, State::new())
}
