use crate::bitreader::Readable;

use super::TypeParse;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CrdtId {
    part1: u8,
    part2: u32,
}

impl TypeParse for CrdtId {
    fn parse(reader: &mut crate::Bitreader<impl Readable>) -> Result<Self, crate::ParseError> {
        Ok(CrdtId {
            part1: reader.read_u8()?, // XXX might be var unit
            part2: reader.read_varuint()?,
        })
    }
}
