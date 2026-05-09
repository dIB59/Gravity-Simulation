//! Graviplex - N-Body Simulation
//!
//! A GPU-accelerated Barnes-Hut n-body simulation built on graviplex-engine.

pub mod nbody;
pub mod par;

pub use nbody::*;

// ─── WASM entry point ────────────────────────────────────────────────────────
//
// Web build flow:
//   cargo build --release --target wasm32-unknown-unknown --no-default-features
//   wasm-bindgen --target web --out-dir pkg ./target/wasm32-unknown-unknown/release/graviplex_sim.wasm
// then load the generated `pkg/graviplex_sim.js` in an HTML page that contains
// `<canvas id="graviplex-canvas">`. The canvas must exist before init() runs.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() {
    use graviplex::prelude::*;

    // Forward Rust panics to the browser console for visibility.
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);

    // Lower particle count for the single-threaded WASM build. Native uses
    // 1 << 19 (524 288); WASM is fine at 1 << 14 (16 384) until we wire
    // wasm-bindgen-rayon for true multi-threading.
    let game = NBodyGame::new(1 << 14);

    if let Err(e) = App::build(game)
        .title("N-Body Simulation")
        .size(1280, 720)
        .run()
    {
        web_sys::console::error_1(&format!("App run failed: {:?}", e).into());
    }
}
