use wasm_bindgen::JsValue;
use web_sys;
use crate::view::{size, view};
use crate::view::size::Size;

pub struct Body {
    child: Box<dyn view::Viewable>,
    pub padding: size::Edge,
    pub margin: size::Edge,
    pub width: Size,
    pub height: Size,
}

impl Body {
    pub fn new(child: Box<dyn view::Viewable>) -> Body {
        Body {
            child,
            padding: size::Edge::new(),
            margin: size::Edge::new(),
            width: Size::MatchParent,
            height: Size::MatchParent,
        }
    }

    pub fn make(self) -> Box<dyn view::Viewable> {
        return Box::new(self);
    }
}

impl view::Viewable for Body {
    fn build(&self, document: &web_sys::Document) -> Result<web_sys::Element, view::Error> {
        let mut body = document.body().ok_or(view::Error::NoBodyFound)? as web_sys::HtmlElement;

        let child = self.child.build(document)?;
        body.append_child(&child);

        // body.style().set_property("background-color", "yellow");

        Ok(web_sys::Element::from(body))
    }
}

pub struct Text {
    str: String,
}

impl Text {
    pub fn new(str: String) -> Text {
        Text { str }
    }

    pub fn make(self) -> Box<dyn view::Viewable> {
        return Box::new(self);
    }
}

impl view::Viewable for Text {
    fn build(&self, document: &web_sys::Document) -> Result<web_sys::Element, view::Error> {
        let val = match document.create_element("p") {
            Ok(e) => e,
            Err(_) => return Err(view::Error::ElementCreation),
        };
        val.set_text_content(Some(&self.str));
        return Ok(val);
    }
}