use wasm_bindgen::JsValue;
use web_sys::Element;

pub trait Buildable {
    fn build(&self, document: &web_sys::Document) -> Result<Element, JsValue>;
}
