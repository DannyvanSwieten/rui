use crate::canvas::{Point, Size};

#[derive(Default)]
pub struct Properties {
    pub position: Point,
    pub size: Size,
    pub has_mouse: bool,
    pub children: Vec<usize>,
}
