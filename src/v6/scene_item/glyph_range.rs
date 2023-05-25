use crate::{shared::pen_color::PenColor, v6::TypeParse, ParseError};

#[derive(Debug, Clone)]
struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl TypeParse for Rectangle {
    fn parse(
        reader: &mut crate::v6::tagged_bit_reader::TaggedBitreader<impl crate::bitreader::Readable>,
    ) -> Result<Self, crate::ParseError> {
        Ok(Rectangle {
            x: reader.bit_reader.read_f64()?,
            y: reader.bit_reader.read_f64()?,
            w: reader.bit_reader.read_f64()?,
            h: reader.bit_reader.read_f64()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GlyphRange {
    start: u32,
    length: u32,
    text: String,
    color: PenColor,
    rectangles: Vec<Rectangle>,
}

impl TypeParse for GlyphRange {
    fn parse(
        reader: &mut crate::v6::tagged_bit_reader::TaggedBitreader<impl crate::bitreader::Readable>,
    ) -> Result<Self, crate::ParseError> {
        let start = reader.read_u32(2)?;
        let length = reader.read_u32(3)?;
        let color = PenColor::try_from(reader.read_u32(4)?)?;
        let text = reader.read_string(5)?;

        let subblock = reader.read_subblock(6)?;
        let rectangles = (0..reader.bit_reader.read_varuint()?)
            .into_iter()
            .map(|_| Rectangle::parse(reader))
            .collect::<Result<Vec<Rectangle>, ParseError>>()?;
        subblock.validate_size(reader)?;

        Ok(GlyphRange {
            start,
            length,
            text,
            color,
            rectangles,
        })
    }
}
