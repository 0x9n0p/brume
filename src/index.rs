use crate::view::size::Size;
use crate::view::size::Size::{MatchParent, Percent, Pixel};
use crate::view::view;
use crate::view::style::{FontWeight, Justify, Width, Height, Padding, Align};
use crate::view::widget::{Body, Button, Column, Row, Text};
use crate::window::page::Page;

pub(crate) struct Login {}

impl Login {
    pub fn new() -> Box<dyn Page> {
        Box::new(Login {})
    }
}

impl Page for Login {
    fn body(&self) -> Box<dyn view::Viewable> {
        Body::new(
            Column::new()
                // .child(Text::new("Login Page").style(FontWeight::bold()))
                .child(Button::primary("Primary Button"))
                .child(Button::text("Text Button"))
                .child(Button::disabled("Disabled Button"))
        )
            .style(Justify::center())
            .style(Align::center())
            .make()
    }
}