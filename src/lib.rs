#![feature(or_patterns)]

pub mod app;
pub mod phasmo;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn wasm_main() {
    // This hook is necessary to get panic messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    app::run()
}
