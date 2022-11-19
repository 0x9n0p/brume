pub struct Edge {
    pub top: u32,
    pub right: u32,
    pub left: u32,
    pub bottom: u32,
}

impl Edge {
    pub fn new() -> Edge {
        Edge{
            top: 0,
            right: 0,
            left: 0,
            bottom: 0
        }
    }
}

pub enum Size {
    MatchParent,
    WrapContent,
    Pixel(i32)
}