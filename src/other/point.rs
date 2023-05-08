use crate::{bitreader::Readable, Parse};

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub direction: f32,
    pub width: f32,
    pub pressure: f32,
}

impl Parse for Point {
    fn parse(
        _version: u32,
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, crate::ParseError> {
        Ok(Point {
            x: reader.read_f32()?,
            y: reader.read_f32()?,
            speed: reader.read_f32()?,
            direction: reader.read_f32()?,
            width: reader.read_f32()?,
            pressure: reader.read_f32()?,
        })
    }
}
