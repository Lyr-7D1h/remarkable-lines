use crate::{
    shared::{pen_color::PenColor, tool::Tool},
    v6::{
        block::{BlockInfo, BlockParse},
        scene_item::point::Point,
    },
    ParseError,
};

#[derive(Debug, Clone)]
pub struct Line {
    color: PenColor,
    tool: Tool,
    points: Vec<Point>,
    thickness_scale: f32,
    starting_length: f32,
}

pub fn point_serialize_size(version: u8) -> Result<u32, ParseError> {
    match version {
        1 => return Ok(0x18),
        2 => return Ok(0x0E),
        _ => {
            return Err(ParseError::unsupported(format!(
                "Block unsupported version: {version}"
            )))
        }
    }
}

impl BlockParse for Line {
    fn parse(
        info: &BlockInfo,
        reader: &mut crate::v6::tagged_bit_reader::TaggedBitreader<impl crate::bitreader::Readable>,
    ) -> Result<Self, crate::ParseError> {
        let tool = Tool::try_from(reader.read_u32(1)?)?;
        let color = PenColor::try_from(reader.read_u32(2)?)?;
        let thinkness_scale = reader.read_f64(3)?;
        let starting_length = reader.read_f32(4)?;

        let subblock = reader.read_subblock(5)?;
        let point_size = point_serialize_size(info.current_version)?;
        if info.size % point_size != 0 {
            return Err(ParseError::invalid(format!(
                "Invalid point data size. {} is not multiple of point_size",
                info.size
            )));
        }
        let points = (0..info.size / point_size)
            .into_iter()
            .map(|_| Point::parse(info, reader))
            .collect::<Result<Vec<Point>, ParseError>>()?;

        todo!()
    }
}
