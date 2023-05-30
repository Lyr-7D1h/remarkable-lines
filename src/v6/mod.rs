pub mod block;
pub mod crdt;
pub mod lwwvalue;
pub mod scene_item;
pub mod scene_tree;
pub mod tagged_bit_reader;

use crate::{bitreader::Readable, ParseError};

use self::tagged_bit_reader::TaggedBitreader;

/// Simplified parsing method only accepting reader
pub trait TypeParse {
    fn parse(reader: &mut TaggedBitreader<impl Readable>) -> Result<Self, ParseError>
    where
        Self: Sized;
}
