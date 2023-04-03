use crate::{Color, Point, Tool};

pub struct Line {
    pub points: Vec<Point>,
    pub tool: Tool,
    pub color: Color,
    pub unknown_line_attribute: u32,
    pub unknown_line_attribute_2: u32,
    pub padding: u32,
    pub brush_size: f32,
}
