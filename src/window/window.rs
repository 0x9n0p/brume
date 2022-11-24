use std::fmt::Error;
use web_sys::HtmlElement;
use crate::view::size::Size;
use crate::view::{view, widget};
use crate::window::page;

pub struct Window {
    page: Box<dyn page::Page>,
    document: web_sys::Document,
}

impl Window {
    pub fn new(page: Box<dyn page::Page>) -> Option<Window> {
        let window = web_sys::window()?;
        let document = window.document()?;
        Some(Window { page, document })
    }

    pub fn build(self) -> Result<Window, view::Error> {
        // TODO: Add header initializer here
        self.page.body().build(&self.document)?;
        Ok(self)
    }
}