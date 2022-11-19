// TODO
pub const MAX_SIZE: Size = Size { height: 0, width: 0 };

pub struct Edge {
    pub top: u32,
    pub right: u32,
    pub left: u32,
    pub bottom: u32,
}

pub struct Size {
    pub height: u32,
    pub width: u32,
}