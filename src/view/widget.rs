use std::borrow::BorrowMut;
use std::collections;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys;
use crate::view::{font, size, style, view};
use crate::view::size::Size;
use crate::view::style::*;
use gloo::{events::EventListener, timers::callback::Timeout};
use web_sys::Node;
use crate::view::color::Colors;
use crate::view::font::Font;

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
        }.style(Background::color(Colors::Custom("#FFFFFF")))
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
        Button::primary(str)
    }

    fn prepare(str: &'static str) -> Button {
        Button {
            str,
            styles: collections::HashMap::default(),
            html_element: None,
        }
            .style(Color::new(Colors::White))
            .style(Cursor::pointer())
            .style(BorderStyle::none())
            .style(FontSize::new(Size::Pixel(14.0)))
            .style(FontFamily::new(Font::SansSerif))
            .style(TextTransform::uppercase())
            .style(BorderRadius::new(Size::Pixel(5.0)))
            .style(Padding::block(Size::Pixel(10.0)))
            .style(Padding::inline(Size::Pixel(45.0)))
            .style(FontWeight::new("600"))
    }

    pub fn primary(str: &'static str) -> Button {
        Button::prepare(str)
            .style(Background::color(Colors::Custom("#6979F8")))
    }

    pub fn text(str: &'static str) -> Button {
        Button::prepare(str)
            .style(Color::new(Colors::Custom("#6979F8")))
            .style(Background::color(Colors::None))
    }

    pub fn disabled(str: &'static str) -> Button {
        Button::prepare(str)
            .style(Color::new(Colors::Lightgray))
            .style(Cursor::default())
            .style(Background::color(Colors::Custom("#FBE4E8")))
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

pub struct Link {
    str: &'static str,
    address: &'static str,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlAnchorElement>,
}

impl Link {
    pub fn new(str: &'static str, address: &'static str) -> Link {
        Link { str, address, styles: Default::default(), html_element: None }
            // Disabled Style
            // Color: #D0C9D6
            // Cursor: default
            .style(Color::new(Colors::Custom("#3F3356")))
            .style(TextDecoration::none())
            .style(FontSize::new(Size::Pixel(16.0)))
            .style(FontFamily::new(Font::SansSerif))
            .style(Cursor::pointer())
    }

    pub fn style(mut self, style: impl Style + 'static) -> Link {
        self.styles.insert(style.name(), Box::new(style));
        self
    }
}

impl view::Viewable for Link {
    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, view::Error> {
        self.html_element = match document.create_element("a") {
            Ok(e) => Some(e.dyn_into::<web_sys::HtmlAnchorElement>().unwrap()),
            Err(_) => return Err(view::Error::ElementCreation),
        };

        let l = self.html_element.as_ref().unwrap();

        l.set_text_content(Some(&self.str));
        l.set_href(self.address);

        for (_, mut style) in &self.styles {
            style.build(&l)?;
        }

        return Ok(l.deref().clone());
    }
}

pub struct Label {
    str: &'static str,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Label {
    pub fn new(str: &'static str) -> Label {
        Label { str, styles: Default::default(), html_element: None }
            .style(FontSize::new(Size::Pixel(16.0)))
            .style(TextTransform::uppercase())
            .style(FontFamily::new(Font::SansSerif))
            .style(BorderRadius::new(Size::Pixel(5.0)))
            .style(Padding::block(Size::Pixel(8.0)))
            .style(Padding::inline(Size::Pixel(14.0)))
            .enable(true)
    }

    pub fn enable(mut self, is: bool) -> Label {
        if is {
            self
                .style(Color::new(Colors::White))
                .style(Background::color(Colors::Custom("#BE52F2")))
        } else {
            self
                .style(Color::new(Colors::Custom("#BE52F2")))
                .style(Background::color(Colors::Custom("#EEDFF2")))
        }
    }

    pub fn style(mut self, style: impl Style + 'static) -> Label {
        self.styles.insert(style.name(), Box::new(style));
        self
    }
}

impl view::Viewable for Label {
    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, view::Error> {
        self.html_element = match document.create_element("div") {
            Ok(e) => Some(e.dyn_into::<web_sys::HtmlElement>().unwrap()),
            Err(_) => return Err(view::Error::ElementCreation),
        };

        let l = self.html_element.as_ref().unwrap();

        l.set_text_content(Some(&self.str));

        for (_, mut style) in &self.styles {
            style.build(&l)?;
        }

        return Ok(l.clone());
    }
}

pub struct Input {
    str: &'static str,
    pub placeholder: &'static str,
    pub enabled: bool,
    styles: collections::HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlInputElement>,
}

impl Input {
    pub fn new(str: &'static str) -> Input {
        Input { str, placeholder: "", enabled: true, styles: Default::default(), html_element: None }
            .style(Width::new(Size::Pixel(210.0)))
            .style(Height::new(Size::Pixel(40.0)))
            .style(FontSize::new(Size::Pixel(15.0)))
            .style(TextTransform::capitalize())
            .style(FontFamily::new(Font::SansSerif))
            .style(BorderRadius::new(Size::Pixel(5.0)))
            .style(Padding::left(Size::Pixel(14.0)))
            .style(BorderStyle::solid())
            .style(BorderWidth::new(Size::Pixel(2.0)))
            .style(BorderColor::new(Colors::Custom("#ECE9F1")))
            .enable(true)
    }

    pub fn enable(mut self, is: bool) -> Input {
        self.enabled = is;
        if is {
            self
                .style(Color::new(Colors::Custom("#1A051D")))
                .style(Background::color(Colors::White))
        } else {
            self
                .style(Color::new(Colors::Custom("#D0C9D6")))
                .style(Background::color(Colors::Custom("#ECE9F1")))
        }
    }

    pub fn style(mut self, style: impl Style + 'static) -> Input {
        self.styles.insert(style.name(), Box::new(style));
        self
    }
}

impl view::Viewable for Input {
    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, view::Error> {
        self.html_element = match document.create_element("input") {
            Ok(e) => Some(e.dyn_into::<web_sys::HtmlInputElement>().unwrap()),
            Err(_) => return Err(view::Error::ElementCreation),
        };

        let l = self.html_element.as_ref().unwrap();

        l.set_value(self.str);
        l.set_placeholder(self.placeholder);
        l.set_disabled(!self.enabled);

        for (_, mut style) in &self.styles {
            style.build(&l)?;
        }

        return Ok(l.deref().clone());
    }
}