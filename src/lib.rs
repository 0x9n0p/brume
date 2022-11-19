pub mod window;
pub mod view;

use window::window::*;
use view::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    Window::build(widget::Text::new("Hello World".to_string()));
}
