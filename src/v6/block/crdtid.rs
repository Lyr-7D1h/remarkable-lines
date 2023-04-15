use super::TypeParse;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CrdtId {
    part1: u8,
    part2: u32,
}

impl TypeParse for CrdtId {
    fn parse<N: std::io::Read>(
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, crate::ParseError> {
        Ok(CrdtId {
            part1: reader.read_u8()?, // TODO might be var unit
            part2: reader.read_varuint()?,
        })
    }
}
