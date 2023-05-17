use crate::bitreader::Readable;

use super::{tagged_bit_reader::TaggedBitreader, TypeParse};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CrdtId {
    part1: u8,
    part2: u32,
}

impl TypeParse for CrdtId {
    fn parse(reader: &mut TaggedBitreader<impl Readable>) -> Result<Self, crate::ParseError> {
        Ok(CrdtId {
            part1: reader.bit_reader.read_u8()?, // XXX might be var unit
            part2: reader.bit_reader.read_varuint()?,
        })
    }
}

impl Default for CrdtId {
    fn default() -> Self {
        Self {
            part1: Default::default(),
            part2: Default::default(),
        }
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
