use std::collections::HashMap;
use std::ops::Deref;
use web_sys;
use wasm_bindgen::JsCast;
use crate::view::color::Colors;
use crate::view::font::Font;
use crate::view::size::Size;
use crate::view::style::*;
use crate::view::view::{Error, Viewable};

pub trait Widget: Styleable {}

pub trait Styleable: Viewable {
    fn style(&mut self, style: impl Style + 'static + Clone) -> &mut Self {
        let style2 = style.clone();
        self.set(Box::new(move |e| {
            style2.build(e).unwrap();
        }));
        self.store_style(style);
        self
    }

    fn store_style(&mut self, style: impl Style + 'static);
}

pub struct Body {
    child: Box<dyn Viewable>,
    styles: HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Body {
    pub fn new(child: impl Viewable + 'static) -> Body {
        Body {
            child: Box::new(child),
            styles: Default::default(),
            html_element: None,
        }
    }

    pub fn make(self) -> Box<dyn Viewable> {
        return Box::new(self);
    }
}

impl Styleable for Body {
    fn store_style(&mut self, style: impl Style + 'static) {
        self.styles.insert(style.name(), Box::new(style));
    }
}

impl Viewable for Body {
    fn get_html_element(&mut self) -> Option<&web_sys::HtmlElement> {
        match &self.html_element {
            Some(e) => { Some(e) }
            None => None
        }
    }

    fn render(&mut self, element: web_sys::HtmlElement, _: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        for (_, mut style) in &self.styles {
            style.build(&element)?;
        }
        Ok(element)
    }

    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        let mut element = document.body().ok_or(Error::NoBodyFound)? as web_sys::HtmlElement;

        let child = self.child.build(document)?;
        element.append_child(&child);

        self.html_element = Some(element.clone());
        self.render(element, document)
    }
}

pub struct Column {
    children: Vec<Box<dyn Viewable>>,
    styles: HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Column {
    pub fn new() -> Column {
        Column {
            children: Vec::default(),
            styles: Default::default(),
            html_element: None,
        }.apply(&|mut column| {
            column
                .style(ContainerDirection::column())
                .style(Justify::start());
            return column;
        })
    }

    pub fn child(mut self, child: impl Viewable + 'static) -> Column {
        self.children.push(Box::new(child));
        self
    }

    pub fn apply(self, f: &dyn Fn(Column) -> Column) -> Self {
        f(self)
    }
}

impl Styleable for Column {
    fn store_style(&mut self, style: impl Style + 'static) {
        self.styles.insert(style.name(), Box::new(style));
    }
}

impl Viewable for Column {
    fn get_html_element(&mut self) -> Option<&web_sys::HtmlElement> {
        match &self.html_element {
            Some(e) => { Some(e) }
            None => None
        }
    }

    fn render(&mut self, element: web_sys::HtmlElement, document: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        for (_, mut style) in &self.styles {
            style.build(&element)?;
        }

        for view in &mut self.children {
            element.append_child(&view.build(document)?.dyn_into::<web_sys::Node>().unwrap());
        }

        self.html_element = Some(element.clone());
        Ok(element)
    }
}

pub struct Row {
    children: Vec<Box<dyn Viewable>>,
    styles: HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Row {
    pub fn new() -> Row {
        Row {
            children: Vec::default(),
            styles: Default::default(),
            html_element: None,
        }
            .apply(&|mut row| {
                row
                    .style(ContainerDirection::row())
                    .style(Justify::start());
                return row;
            })
    }

    pub fn child(mut self, child: impl Viewable + 'static) -> Row {
        self.children.push(Box::new(child));
        self
    }

    pub fn apply(self, f: &dyn Fn(Row) -> Row) -> Self {
        f(self)
    }
}

impl Styleable for Row {
    fn store_style(&mut self, style: impl Style + 'static) {
        self.styles.insert(style.name(), Box::new(style));
    }
}

impl Viewable for Row {
    fn get_html_element(&mut self) -> Option<&web_sys::HtmlElement> {
        match &self.html_element {
            Some(e) => { Some(e) }
            None => None
        }
    }

    fn render(&mut self, element: web_sys::HtmlElement, document: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        for (_, mut style) in &self.styles {
            style.build(&element)?;
        }

        for view in &mut self.children {
            element.append_child(&view.build(document)?.dyn_into::<web_sys::Node>().unwrap());
        }

        self.html_element = Some(element.clone());
        Ok(element)
    }
}

pub struct Text {
    str: &'static str,
    styles: HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Text {
    pub fn new(str: &'static str) -> Text {
        Text { str, styles: Default::default(), html_element: None }
    }

    pub fn apply(self, f: &dyn Fn(Text) -> Text) -> Self {
        f(self)
    }
}

impl Styleable for Text {
    fn store_style(&mut self, style: impl Style + 'static) {
        self.styles.insert(style.name(), Box::new(style));
    }
}

impl Viewable for Text {
    fn get_tag(&self) -> &'static str { return "p"; }

    fn get_html_element(&mut self) -> Option<&web_sys::HtmlElement> {
        match &self.html_element {
            Some(e) => { Some(e) }
            None => None
        }
    }

    fn render(&mut self, element: web_sys::HtmlElement, _: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        element.set_text_content(Some(&self.str));

        for (_, mut style) in &self.styles {
            style.build(&element)?;
        }

        self.html_element = Some(element.clone());
        Ok(element)
    }
}

pub struct Button {
    str: &'static str,
    styles: HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Button {
    pub fn new(str: &'static str) -> Button {
        Button::primary(str)
    }

    fn prepare(str: &'static str) -> Button {
        Button {
            str,
            styles: HashMap::default(),
            html_element: None,
        }
            .apply(&|mut button| {
                button
                    .style(Color::new(Colors::White))
                    .style(Cursor::pointer())
                    .style(BorderStyle::none())
                    .style(FontSize::new(Size::Pixel(14.0)))
                    .style(FontFamily::new(Font::SansSerif))
                    .style(TextTransform::uppercase())
                    .style(BorderRadius::new(Size::Pixel(5.0)))
                    .style(Padding::block(Size::Pixel(10.0)))
                    .style(Padding::inline(Size::Pixel(45.0)))
                    .style(FontWeight::new("600"));
                return button;
            })
    }

    pub fn primary(str: &'static str) -> Button {
        Button::prepare(str)
            .apply(&|mut button| {
                button
                    .style(Background::color(Colors::Custom("#6979F8")));
                return button;
            })
    }

    pub fn text(str: &'static str) -> Button {
        Button::prepare(str)
            .apply(&|mut button| {
                button
                    .style(Color::new(Colors::Custom("#6979F8")))
                    .style(Background::color(Colors::None));
                return button;
            })
    }

    pub fn disabled(str: &'static str) -> Button {
        Button::prepare(str)
            .apply(&|mut button| {
                button
                    .style(Color::new(Colors::Lightgray))
                    .style(Cursor::default())
                    .style(Background::color(Colors::Custom("#FBE4E8")));
                return button;
            })
    }

    pub fn apply(self, f: &dyn Fn(Button) -> Button) -> Self {
        f(self)
    }
}

impl Styleable for Button {
    fn store_style(&mut self, style: impl Style + 'static) {
        self.styles.insert(style.name(), Box::new(style));
    }
}

impl Viewable for Button {
    fn get_tag(&self) -> &'static str { return "button"; }

    fn get_html_element(&mut self) -> Option<&web_sys::HtmlElement> {
        match &self.html_element {
            Some(e) => { Some(e) }
            None => None
        }
    }

    fn render(&mut self, element: web_sys::HtmlElement, _: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        element.set_text_content(Some(&self.str));

        for (_, mut style) in &self.styles {
            // style.build(&*Rc::clone(&btn))?;
            style.build(&element)?;
        }

        self.html_element = Some(element.clone());
        Ok(element)
    }
}

pub struct Link {
    str: &'static str,
    address: &'static str,
    styles: HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlAnchorElement>,
}

impl Link {
    pub fn new(str: &'static str, address: &'static str) -> Link {
        Link { str, address, styles: Default::default(), html_element: None }
            // Disabled Style
            // Color: #D0C9D6
            // Cursor: default
            .apply(&|mut link| {
                link.style(Color::new(Colors::Custom("#3F3356")))
                    .style(TextDecoration::none())
                    .style(FontSize::new(Size::Pixel(16.0)))
                    .style(FontFamily::new(Font::SansSerif))
                    .style(Cursor::pointer());
                return link;
            })
    }

    pub fn apply(self, f: &dyn Fn(Link) -> Link) -> Self {
        f(self)
    }
}

impl Styleable for Link {
    fn store_style(&mut self, style: impl Style + 'static) {
        self.styles.insert(style.name(), Box::new(style));
    }
}

impl Viewable for Link {
    fn get_tag(&self) -> &'static str { return "a"; }

    fn get_html_element(&mut self) -> Option<&web_sys::HtmlElement> {
        match &self.html_element {
            Some(e) => { Some(e) }
            None => None
        }
    }

    fn render(&mut self, element: web_sys::HtmlElement, _: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        let l = element.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();

        l.set_text_content(Some(&self.str));
        l.set_href(self.address);

        for (_, mut style) in &self.styles {
            style.build(&l)?;
        }

        self.html_element = Some(l.clone());
        Ok(l.deref().clone())
    }
}

pub struct Label {
    str: &'static str,
    styles: HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlElement>,
}

impl Label {
    pub fn new(str: &'static str) -> Label {
        Label { str, styles: Default::default(), html_element: None }
            .apply(&|mut label| {
                label
                    .style(FontSize::new(Size::Pixel(16.0)))
                    .style(TextTransform::uppercase())
                    .style(FontFamily::new(Font::SansSerif))
                    .style(BorderRadius::new(Size::Pixel(5.0)))
                    .style(Padding::block(Size::Pixel(8.0)))
                    .style(Padding::inline(Size::Pixel(14.0)))
                    .enable(true);
                return label;
            })
    }

    pub fn enable(&mut self, is: bool) -> &Self {
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

    pub fn apply(self, f: &dyn Fn(Label) -> Label) -> Self {
        f(self)
    }
}

impl Styleable for Label {
    fn store_style(&mut self, style: impl Style + 'static) {
        self.styles.insert(style.name(), Box::new(style));
    }
}

impl Viewable for Label {
    fn get_html_element(&mut self) -> Option<&web_sys::HtmlElement> {
        match &self.html_element {
            Some(e) => { Some(e) }
            None => None
        }
    }

    fn render(&mut self, element: web_sys::HtmlElement, _: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        element.set_text_content(Some(&self.str));

        for (_, mut style) in &self.styles {
            // style.build(&*Rc::clone(&btn))?;
            style.build(&element)?;
        }

        self.html_element = Some(element.clone());
        Ok(element)
    }
}

pub struct Input {
    str: &'static str,
    pub placeholder: &'static str,
    pub enabled: bool,
    styles: HashMap<&'static str, Box<dyn Style>>,
    html_element: Option<web_sys::HtmlInputElement>,
}

impl Input {
    pub fn new(str: &'static str) -> Input {
        Input { str, placeholder: "", enabled: true, styles: Default::default(), html_element: None }
            .apply(&|mut input| {
                input
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
                    .enable(true);
                return input;
            })
    }

    pub fn enable(&mut self, is: bool) -> &Self {
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

    pub fn apply(self, f: &dyn Fn(Input) -> Input) -> Self {
        f(self)
    }
}

impl Styleable for Input {
    fn store_style(&mut self, style: impl Style + 'static) {
        self.styles.insert(style.name(), Box::new(style));
    }
}

impl Viewable for Input {
    fn get_tag(&self) -> &'static str { return "input"; }

    fn get_html_element(&mut self) -> Option<&web_sys::HtmlElement> {
        match &self.html_element {
            Some(e) => { Some(e) }
            None => None
        }
    }

    fn render(&mut self, element: web_sys::HtmlElement, _: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        let input = element.dyn_into::<web_sys::HtmlInputElement>().unwrap();

        input.set_value(self.str);
        input.set_placeholder(self.placeholder);
        input.set_disabled(!self.enabled);

        for (_, mut style) in &self.styles {
            style.build(&input)?;
        }

        self.html_element = Some(input.clone());
        Ok(input.deref().clone())
    }
}