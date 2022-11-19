use wasm_bindgen::JsValue;
use web_sys::Element;
use super::*;

pub struct View {
    pub parent: Box<dyn buildable::Buildable>,
    pub child: Box<dyn buildable::Buildable>,
    pub padding: size::Edge,
    pub margin: size::Edge,
    pub size: size::Size,
}

impl buildable::Buildable for View {
    fn build(&self, document: &web_sys::Document) -> Result<Element, JsValue> {
        todo!()
    }
}