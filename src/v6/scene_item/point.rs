use std::f32::consts::PI;

use crate::v6::block::BlockParse;

#[derive(Debug, Clone)]
pub struct Point {
    x: f32,
    y: f32,
    speed: u32,
    direction: u32,
    width: u32,
    pressure: u32,
}

impl BlockParse for Point {
    fn parse(
        info: &crate::v6::block::BlockInfo,
        reader: &mut crate::v6::tagged_bit_reader::TaggedBitreader<impl crate::bitreader::Readable>,
    ) -> Result<Self, crate::ParseError> {
        let x = reader.bit_reader.read_f32()?;
        let y = reader.bit_reader.read_f32()?;
        if info.current_version == 1 {
            let speed = reader.bit_reader.read_f32()? * 4;
            let direction = 255 * reader.bit_reader.read_f32()? / (PI * 2);
            let width = u32::try_from(reader.bit_reader.read_f32()? * 4)?;
            let pressure = reader.bit_reader.read_f32() * 255;
            return Ok(Point {
                x,
                y,
                speed,
                direction,
                width,
                pressure,
            });
        } else {
            let speed = reader.bit_reader.read_u16();
            let width = reader.bit_reader.read_u16();
            let direction = reader.bit_reader.read_u8();
            let pressure = reader.bit_reader.read_u8();
            return Ok(Point {
                x,
                y,
                speed,
                direction,
                width,
                pressure,
            });
        }
    }
}
