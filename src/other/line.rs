use crate::bitreader::Readable;
use crate::shared::pen_color::PenColor;
use crate::shared::tool::Tool;
use crate::Parse;
use crate::ParseError;

use super::point::Point;

#[derive(Debug)]
pub struct Line {
    pub points: Vec<Point>,
    pub tool: Tool,
    pub color: PenColor,
    pub brush_size: f32,
}

impl Parse for Line {
    fn parse(
        version: u32,
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, crate::ParseError> {
        let tool = Tool::try_from(reader.read_u32()?)?;
        let color = PenColor::try_from(reader.read_u32()?)?;
        reader.read_u32()?; // Skip unknown value
        let brush_size = reader.read_f32()?;
        if version >= 5 {
            reader.read_u32()?; // Skip unkown value
        }
        let amount_points = reader.read_u32()?;

        Ok(Line {
            tool,
            color,
            brush_size,
            points: (0..amount_points)
                .map(|_| {
                    Ok(Point {
                        x: reader.read_f32()?,
                        y: reader.read_f32()?,
                        speed: reader.read_f32()?,
                        direction: reader.read_f32()?,
                        width: reader.read_f32()?,
                        pressure: reader.read_f32()?,
                    })
                })
                .collect::<Result<Vec<Point>, ParseError>>()?,
        })
    }
}
