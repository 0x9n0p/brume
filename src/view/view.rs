use wasm_bindgen::JsValue;
use web_sys::Element;

pub trait Viewable {
    fn build(&self, document: &web_sys::Document) -> Result<Element, Error>;
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("Failed to get body")]
    NoBodyFound,
    #[error("Failed to create element")]
    ElementCreation,
}