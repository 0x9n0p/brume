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
        // let window = web_sys::window()?;
        // let document = window.document()?;
        //
        // let new_div = document.create_element("div").ok()?;
        //
        // // let new_p = document.create_element("p").ok()?;
        // // (document.body()? as HtmlElement).set_title("hello");
        // // (document.body()? as HtmlElement).style();
        // // (document.body()? as HtmlElement).style().set_property("background-color", "yellow");
        // new_p.set_text_content(Some(&(document.body()? as HtmlElement).title()));
        // // new_p.style().set_property("background-color", "red");
        // new_div.append_child(&new_p);
        //
        // // document.body()?.style().set_property("background-color", "red");
        //
        // let val = body.build(&document).ok()?;
        // document.body()?.append_child(&val);
        // document.body()?.append_child(&new_div);
        //
        // // (document.body()? as HtmlElement).style.set_property("background-color", "black");
        //
        // // let val = document.create_element("p")?;
        // // val.set_text_content(Some("Hello from Rust!"));
        // // body.append_child(&val)?;

        // TODO: Add header initializer here
        self.page.body().build(&self.document)?;
        Ok(self)
    }
}