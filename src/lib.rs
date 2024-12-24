mod state;
mod handler;
// cant use the module name `app` for some reason, rust-analyzer will only recognize the file if the module name is capitalized

use handler::App;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub async fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    console::log_1(&"testing".into());

    App::start().await;
}