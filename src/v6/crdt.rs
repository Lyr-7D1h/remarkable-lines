use std::collections::HashMap;

use crate::bitreader::Readable;

use super::{tagged_bit_reader::TaggedBitreader, TypeParse};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct CrdtId {
    pub part1: u8,
    pub part2: u32,
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

#[derive(Debug, Clone)]
pub struct CrdtSequenceItem<N> {
    pub item_id: CrdtId,
    pub left_id: CrdtId,
    pub right_id: CrdtId,
    pub deleted_length: u32,
    pub value: N,
}

#[derive(Debug, Clone)]
pub struct CrdtSequence<N> {
    pub items: HashMap<CrdtId, CrdtSequenceItem<N>>,
}

impl<N> CrdtSequence<N> {
    pub fn new(items: HashMap<CrdtId, CrdtSequenceItem<N>>) -> Self {
        Self { items }
    }

    pub fn push(&mut self, item: CrdtSequenceItem<N>) -> Option<CrdtSequenceItem<N>> {
        self.items.insert(item.item_id.clone(), item)
    }
}

impl<N> Default for CrdtSequence<N> {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<N> FromIterator<CrdtSequenceItem<N>> for CrdtSequence<N> {
    fn from_iter<T: IntoIterator<Item = CrdtSequenceItem<N>>>(iter: T) -> Self {
        let items = iter.into_iter().map(|i| (i.item_id, i)).collect();
        Self::new(items)
    }
}
