use crate::view::size::Size;
use crate::view::style::{Align, Justify, Margin};
use crate::view::view::Viewable;
use crate::view::widget::{Body, Button, Column, Input, Styleable, Title};
use crate::window::page::Page;

pub(crate) struct Login {}

impl Login {
    pub fn new() -> Box<dyn Page> {
        Box::new(Login {})
    }
}

impl Page for Login {
    fn body(&self) -> Box<dyn Viewable> {
        Body::new(
            Column::new()
                .child(Title::h2("Login"))
                .child(Input::new("Username"))
                .child(Input::new("Password"))
                .child(Button::new("Submit"))
        ).apply(&|mut body| {
            body.style(Justify::center())
                .style(Align::center());
            return body;
        })
            .make()
    }
}