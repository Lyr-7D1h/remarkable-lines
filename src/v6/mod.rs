pub mod block;
pub mod crdt;
pub mod lwwvalue;
pub mod scene_item;
pub mod scene_tree;
pub mod tagged_bit_reader;

use block::Block;
use tagged_bit_reader::TaggedBitreader;

use crate::{bitreader::Readable, ParseError};

/// Simplified parsing method only accepting reader
pub trait TypeParse {
    fn parse(reader: &mut TaggedBitreader<impl Readable>) -> Result<Self, ParseError>
    where
        Self: Sized;
}
