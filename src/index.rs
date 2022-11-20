use web_sys::Element;
use crate::view::view;
use crate::view::widget::{Body, Text};
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
            Text::new("Body initialized".to_string()).make()
        ).make()
    }
}