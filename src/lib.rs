mod window;

use window::window::Window;
use window::view::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn build(name: &str) {
    Window::build(widget::Text::new("Hello World".to_string()));
    alert(&format!("Hello {}!", name));
}

// #[wasm_bindgen]
// pub fn run() -> Result<(), JsValue> {
//     // Use `web_sys`'s global `window` function to get a handle on the global
//     // window object.
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");
//
//     // Manufacture the element we're gonna append
//     let val = document.create_element("p")?;
//     val.set_text_content(Some("Hello from Rust!"));
//
//     body.append_child(&val)?;
//
//     Ok(())
// }
