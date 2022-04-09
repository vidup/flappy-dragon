use bracket_lib::prelude::*;
mod game_objects;
mod state;

/** GAME CONSTANTS */
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 16.67;

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Bracket Terminal")
        .build()?;

    main_loop(context, state::State::new())
}
