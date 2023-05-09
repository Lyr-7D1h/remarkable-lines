use crate::bitreader::Readable;

use super::TypeParse;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CrdtId {
    part1: u8,
    part2: u32,
}

impl Default for CrdtId {
    fn default() -> Self {
        Self {
            part1: Default::default(),
            part2: Default::default(),
        }
    }
}

impl TypeParse for CrdtId {
    fn parse(reader: &mut crate::Bitreader<impl Readable>) -> Result<Self, crate::ParseError> {
        Ok(CrdtId {
            part1: reader.read_u8()?, // XXX might be var unit
            part2: reader.read_varuint()?,
        })
    }
}

#[derive(Debug)]
pub struct CrdtSequenceItem<N> {
    item_id: CrdtId,
    left_id: CrdtId,
    right_id: CrdtId,
    deleted_length: CrdtId,
    value: N,
}

#[derive(Debug)]
pub struct CrdtSequence<N> {
    items: Vec<CrdtSequenceItem<N>>,
}

impl<N> CrdtSequence<N> {
    pub fn new(items: Vec<CrdtSequenceItem<N>>) -> Self {
        Self { items }
    }
}

impl<N> Default for CrdtSequence<N> {
    fn default() -> Self {
        Self { items: vec![] }
    }
}
