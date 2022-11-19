use std::fmt::Error;
use web_sys::HtmlElement;
use crate::view::size::Size;
use crate::view::{view, widget};

pub struct Window {
    body: Box<dyn view::Viewable>,
    document: web_sys::Document,
}

impl Window {
    pub fn new() -> Option<Window> {
        let window = web_sys::window()?;
        let document = window.document()?;
        Some(Window { body: Box::new(widget::Text::new("Hello".to_string())), document })
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
        self.body.build(&self.document)?;
        Ok(self)
    }

    pub fn body(mut self, body: Box<dyn view::Viewable>) -> Window {
        self.body = body;
        return self;
    }
}