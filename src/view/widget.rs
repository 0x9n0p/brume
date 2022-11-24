use std::borrow::BorrowMut;
use std::collections;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys;
use crate::view::{size, style, view};
use crate::view::size::Size;
use crate::view::style::*;
use gloo::{events::EventListener, timers::callback::Timeout};
use crate::view::color::Colors;

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
        }
            .style(Width::new(Size::Pixel(250.0)))
            .style(Height::new(Size::Pixel(50.0)))
            .style(Color::new(Colors::White))
            .style(BackgroundColor::new(Colors::RoyalBlue))
            .style(BorderRadius::new(Size::Pixel(9.0)))
            .style(TextTransform::capitalize())
            .style(Border::none())
            .style(Outline::none())
            .style(Padding::block(Size::Pixel(13.0)))
            .style(FontWeight::new("500"))
            .style(LetterSpacing::new(Size::Pixel(1.2)))
            .style(Cursor::pointer())
    }

    pub fn style(mut self, style: impl Style + 'static) -> Button {
        self.styles.insert(style.name(), Box::new(style));
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

        return Ok(btn.clone());
    }
}