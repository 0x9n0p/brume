use web_sys::Element;
use crate::view::{style, view};
use crate::view::size::Size;
use crate::view::widget::{Body, Button, Text};
use crate::window::page;

pub(crate) struct Page {}

impl Page {
    pub fn new() -> Box<dyn page::Page> {
        Box::new(Page {})
    }
}

impl page::Page for Page {
    fn body(&self) -> Box<dyn view::Viewable> {
        Body::new(
            // Text::new("hello".to_string())
            Button::new("name".to_string())
        ).make()
    }
}