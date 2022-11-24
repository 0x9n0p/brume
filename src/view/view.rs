use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlElement};

pub trait Viewable {
    fn build(&mut self, document: &web_sys::Document) -> Result<HtmlElement, Error>;
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("Failed to get body")]
    NoBodyFound,
    #[error("Failed to create element")]
    ElementCreation,
}