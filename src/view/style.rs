use crate::view::{size, view};

pub trait Style {
    fn name(&self) -> &'static str where Self: Sized;
    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error>;
}

pub struct Height {
    val: size::Size,
}

impl Height {
    pub fn new(val: size::Size) -> Height {
        Height {
            val,
        }
    }
}

impl Style for Height {
    fn name(&self) -> &'static str {
        return "HEIGHT";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        // self.apply();
        element.style().set_property("height", &self.val.to_string());
        return Ok(());
    }
}