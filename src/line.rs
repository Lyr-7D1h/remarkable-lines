use crate::{Color, Point, Tool};

#[derive(Debug)]
pub struct Line {
    pub points: Vec<Point>,
    pub tool: Tool,
    pub color: Color,
    pub brush_size: f32,
}
