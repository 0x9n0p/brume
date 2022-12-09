use web_sys;
use wasm_bindgen::JsCast;

pub trait Viewable {
    fn get_tag(&self) -> &'static str { return "div"; }
    fn get_html_element(&mut self) -> Option<&web_sys::HtmlElement>;

    fn set(&mut self, f: Box<dyn Fn(&web_sys::HtmlElement)>) {
        match self.get_html_element() {
            Some(e) => { f(e) }
            None => {}
        };
    }

    fn render(&mut self, element: web_sys::HtmlElement, _: &web_sys::Document)
              -> Result<web_sys::HtmlElement, Error> { Ok(element) }

    fn build(&mut self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, Error> {
        let html_element = match document.create_element(self.get_tag()) {
            Ok(e) => e.dyn_into::<web_sys::HtmlElement>().unwrap(),
            Err(_) => return Err(Error::ElementCreation),
        };
        self.render(html_element, document)
    }
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("Failed to get body")]
    NoBodyFound,
    #[error("Failed to create element")]
    ElementCreation,
}