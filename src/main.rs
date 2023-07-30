// Property to hide the console window in all builds:
// #![windows_subsystem = "windows"]

// Property to hide the console window only in the release builds only:
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tetra::{Context, ContextBuilder};

use anyhow::Context as anyhow_context;

mod game;
mod util;

use crate::game::GameState;
use crate::util::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> anyhow::Result<()> {
    // create the game context object,
    // to hold all global state, such as manage window settings and connections
    // to the underlying graphics/audio/input hardware
    let game_context: Result<Context, tetra::TetraError> =
        ContextBuilder::new("Pong-Game", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
            .quit_on_escape(true)
            .show_mouse(true)
            .build();

    match game_context {
        Ok(mut context) => {
            // main function will be return the result of method run()
            context.run(|ctx| {
                // build the GameState object with all the required things for the game
                let game_state = GameState::new(ctx)
                    .with_context(|| "Something went wrong while init the game.")?;

                Ok(game_state)
            })
        }
        Err(err) => Err(anyhow::anyhow!(format!(
            "Failed to init Pong-Game on this platform: Err: {}",
            err
        ))),
    }
}
