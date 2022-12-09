use crate::view::size::Size;
use crate::view::size::Size::{MatchParent, Percent, Pixel};
use crate::view::view;
use crate::view::style::{FontWeight, Justify, Width, Height, Padding, Align};
use crate::view::widget::{Body, Text, Styleable, Column};
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
                .child(Text::new("Login Page"))
                .child(Text::new("Login Page"))
            //         // .child(Button::text("Text Button"))
            //         // .child(Button::disabled("Disabled Button"))
            //         // .child(Link::new("my link", "address"))
            //         // .child(Label::new("LABEL"))
            //         // .child(Label::new("LABEL").enable(false))
            //         // .child(Input::new("hello").enable(true))
            //         // .child(Input::new("hello").enable(false))
        )
            // .style(Justify::center())
            // .style(Align::center())
            .make()
    }
}