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
use web_sys::Node;
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

pub struct Column {
    children: Vec<Box<dyn view::Viewable>>,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Column {
    pub fn new() -> Column {
        Column {
            children: Vec::default(),
            styles: Default::default(),
            html_element: None,
        }
            .style(ContainerDirection::column())
            .style(Justify::start())
    }

    pub fn child(mut self, child: impl view::Viewable + 'static) -> Column {
        self.children.push(Box::new(child));
        self
    }

    pub fn style(mut self, style: impl Style + 'static) -> Column {
        self.styles.insert(style.name(), Box::new(style));
        self
    }

    pub fn make(self) -> Box<dyn view::Viewable> {
        return Box::new(self);
    }
}

impl view::Viewable for Column {
    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, view::Error> {
        self.html_element = match document.create_element("div") {
            Ok(e) => Some(e.dyn_into::<web_sys::HtmlElement>().unwrap()),
            Err(_) => return Err(view::Error::ElementCreation),
        };

        let clmn = self.html_element.as_ref().unwrap();

        for (_, mut style) in &self.styles {
            style.build(&clmn)?;
        }

        for view in &mut self.children {
            clmn.append_child(&view.build(document)?.dyn_into::<web_sys::Node>().unwrap());
        }

        return Ok(clmn.clone());
    }
}

pub struct Row {
    children: Vec<Box<dyn view::Viewable>>,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Row {
    pub fn new() -> Row {
        Row {
            children: Vec::default(),
            styles: Default::default(),
            html_element: None,
        }
            .style(ContainerDirection::row())
            .style(Justify::start())
    }

    pub fn child(mut self, child: impl view::Viewable + 'static) -> Row {
        self.children.push(Box::new(child));
        self
    }

    pub fn style(mut self, style: impl Style + 'static) -> Row {
        self.styles.insert(style.name(), Box::new(style));
        self
    }

    pub fn make(self) -> Box<dyn view::Viewable> {
        return Box::new(self);
    }
}

impl view::Viewable for Row {
    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, view::Error> {
        self.html_element = match document.create_element("div") {
            Ok(e) => Some(e.dyn_into::<web_sys::HtmlElement>().unwrap()),
            Err(_) => return Err(view::Error::ElementCreation),
        };

        let clmn = self.html_element.as_ref().unwrap();

        for (_, mut style) in &self.styles {
            style.build(&clmn)?;
        }

        for view in &mut self.children {
            clmn.append_child(&view.build(document)?.dyn_into::<web_sys::Node>().unwrap());
        }

        return Ok(clmn.clone());
    }
}

pub struct Text {
    str: &'static str,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Text {
    pub fn new(str: &'static str) -> Text {
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
    str: &'static str,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Button {
    pub fn new(str: &'static str) -> Button {
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