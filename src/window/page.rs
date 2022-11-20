use crate::view::view;

pub trait Page {
    // TODO
    // fn head(&self) -> ???;
    fn body(&self) -> Box<dyn view::Viewable>;
}