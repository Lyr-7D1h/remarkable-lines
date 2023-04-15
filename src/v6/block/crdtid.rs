use crate::ParseError;

use super::{
    tag::{Tag, TagType},
    TypeParse,
};

#[derive(Debug)]
pub struct CrdtId {
    part1: u8,
    part2: u32,
}

impl TypeParse for CrdtId {
    fn parse<N: std::io::Read>(
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, crate::ParseError> {
        let tag = Tag::parse(reader)?;

        if tag.tag_type != TagType::ID {
            return Err(ParseError::invalid("Tag is not an ID"));
        }

        Ok(CrdtId {
            part1: reader.read_u8()?, // TODO might be var unit
            part2: reader.read_varuint()?,
        })
    }
}
