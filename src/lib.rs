pub mod window;
pub mod view;

use window::window::*;
use view::size::{Size, Edge};
use view::widget::{Body, Text};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    Window::new().expect("Failed to init window")
        .body(Box::new(Body::new( Box::new(Text::new("Hello".to_string())))))
        .build().expect("Failed to build window");
}
