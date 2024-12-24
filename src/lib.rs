mod state;

use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::state::State;


#[wasm_bindgen(start)]
pub async fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    console::log_1(&"testing".into());

    let window = web_sys::window().expect("could not obtain window!");
    let document = window.document().expect("could not obtain result!");
    
    let state = State::init(&document).await;
}