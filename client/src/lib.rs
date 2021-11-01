mod app;
mod header;
mod scan;
mod display;

pub use app::*;
pub use header::*;
pub use scan::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
