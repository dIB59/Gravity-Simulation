//! Graviplex - N-Body Simulation Binary

use graviplex::prelude::*;
use graviplex_sim::NBodyGame;

fn main() -> Result<(), winit::error::EventLoopError> {
    // 1M particles (1 << 20)
    let game = NBodyGame::new(1 << 19);
    App::build(game)
        .title("N-Body Simulation")
        .size(1280, 720)
        .run()?;
    Ok(())
}
