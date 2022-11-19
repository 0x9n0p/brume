use crate::view::buildable;

pub struct Window {
    document: web_sys::Document,
}

impl Window {
    pub fn build(body: Box<dyn buildable::Buildable>) -> Option<Window> {
        let window = web_sys::window()?;
        let document = window.document()?;

        let val = body.build(&document).ok()?;
        document.body()?.append_child(&val);

        // let val = document.create_element("p")?;
        // val.set_text_content(Some("Hello from Rust!"));
        // body.append_child(&val)?;

        return Some(Window { document });
    }
}