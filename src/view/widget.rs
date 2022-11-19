use wasm_bindgen::JsValue;
use web_sys::Element;
use super::*;

pub struct Body {
    title: String,
    view: view::View,
}

impl Body {
    pub fn new(title: String, size: size::Size, child: Box<dyn buildable::Buildable>) -> Box<dyn buildable::Buildable> {
        Box::new(Body {
            title,
            view: view::View {
                parent: Space::new(size::Size { height: 0, width: 0 }),
                child,
                padding: size::Edge {
                    top: 0,
                    right: 0,
                    left: 0,
                    bottom: 0,
                },
                margin: size::Edge {
                    top: 0,
                    right: 0,
                    left: 0,
                    bottom: 0,
                },
                size,
            },
        })
    }
}

impl buildable::Buildable for Body {
    fn build(&self, document: &web_sys::Document) -> Result<Element, JsValue> {
        todo!()
    }
}

pub struct Space {
    size: size::Size,
}

impl Space {
    pub fn new(size: size::Size) -> Box<dyn buildable::Buildable> {
        Box::new(Space { size })
    }
}

impl buildable::Buildable for Space {
    fn build(&self, document: &web_sys::Document) -> Result<Element, JsValue> {
        todo!()
    }
}

pub struct Text {
    str: String,
}

impl Text {
    pub fn new(str: String) -> Box<dyn buildable::Buildable> {
        Box::new(Text { str })
    }
}

impl buildable::Buildable for Text {
    fn build(&self, document: &web_sys::Document) -> Result<Element, JsValue> {
        let val = document.create_element("p")?;
        val.set_text_content(Some(&self.str));
        return Ok(val);
    }
}
