use std::borrow::BorrowMut;
use std::collections;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys;
use crate::view::{size, style, view};
use crate::view::size::Size;
use crate::view::style::{Height, Style};
use gloo::{events::EventListener, timers::callback::Timeout};

pub struct Body {
    child: Box<dyn view::Viewable>,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Body {
    pub fn new(child: impl view::Viewable + 'static) -> Body {
        Body {
            child: Box::new(child),
            styles: Default::default(),
            html_element: None,
        }
    }

    pub fn style(mut self, style: impl Style + 'static) -> Body {
        self.styles.insert(style.name(), Box::new(style));
        self
    }

    pub fn styles(mut self, styles: Vec<impl Style + 'static>) -> Body {
        for style in styles {
            self.styles.insert(style.name(), Box::new(style));
        }
        self
    }

    pub fn make(self) -> Box<dyn view::Viewable> {
        return Box::new(self);
    }
}

impl view::Viewable for Body {
    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, view::Error> {
        let mut body = document.body().ok_or(view::Error::NoBodyFound)? as web_sys::HtmlElement;

        for (_, mut style) in &self.styles {
            style.build(&body)?;
        }

        let child = self.child.build(document)?;
        body.append_child(&child);

        Ok(body)
    }
}

pub struct Text {
    str: String,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Text {
    pub fn new(str: String) -> Text {
        Text { str, styles: Default::default(), html_element: None }
    }

    pub fn style(mut self, style: impl Style + 'static) -> Text {
        self.styles.insert(style.name(), Box::new(style));
        self
    }

    pub fn styles(mut self, styles: Vec<impl Style + 'static>) -> Text {
        for style in styles {
            self.styles.insert(style.name(), Box::new(style));
        }
        self
    }
}

impl view::Viewable for Text {
    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, view::Error> {
        self.html_element = match document.create_element("p") {
            Ok(e) => Some(e.dyn_into::<web_sys::HtmlElement>().unwrap()),
            Err(_) => return Err(view::Error::ElementCreation),
        };

        let txt = self.html_element.as_ref().unwrap();

        txt.set_text_content(Some(&self.str));

        for (_, mut style) in &self.styles {
            style.build(&txt)?;
        }

        return Ok(txt.clone());
    }
}

pub struct Button {
    str: String,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Button {
    pub fn new(str: String) -> Button {
        Button {
            str,
            styles: collections::HashMap::default(),
            html_element: None,
        }.styles(vec![
            Height::new(Size::Pixel(50)),
        ])
    }

    pub fn style(mut self, style: impl Style + 'static) -> Button {
        self.styles.insert(style.name(), Box::new(style));
        self
    }

    pub fn styles(mut self, styles: Vec<impl Style + 'static>) -> Button {
        for style in styles {
            self.styles.insert(style.name(), Box::new(style));
        }
        self
    }
}

impl view::Viewable for Button {
    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, view::Error> {
        self.html_element = match document.create_element("button") {
            Ok(e) => Some(e.dyn_into::<web_sys::HtmlElement>().unwrap()),
            Err(_) => return Err(view::Error::ElementCreation),
        };

        let btn = self.html_element.as_ref().unwrap();

        btn.set_text_content(Some(&self.str));

        for (_, mut style) in &self.styles {
            // style.build(&*Rc::clone(&btn))?;
            style.build(&btn)?;
        }

        btn.style().set_property("color", "#FFFFFF");
        btn.style().set_property("background-color", "royalBlue");
        btn.style().set_property("border-radius", "9px");
        btn.style().set_property("text-transform", "capitalize");
        btn.style().set_property("border", "none");
        btn.style().set_property("outline", "none");
        btn.style().set_property("padding-block", "13px");
        btn.style().set_property("width", "100%");
        btn.style().set_property("max-width", "250px");
        // btn.style().set_property("height", "50px");
        btn.style().set_property("font-weight", "500");
        btn.style().set_property("letter-spacing", "1.2px");
        btn.style().set_property("cursor", "pointer");
        // btn.style().set_property("margin", "auto");

        return Ok(btn.clone());
    }
}