use wasm_bindgen::prelude::*;
use rltk::{Rltk, GameState};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
}

// entry point for the wasm-pack module
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue>
{
    let _e = main();
    Ok(())
}
