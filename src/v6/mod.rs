mod crdt;
mod group;
mod lwwvalue;
mod scene_tree;
mod subblock;
mod tagged_bit_reader;
mod text;

pub use scene_tree::SceneTree;

mod block;
pub use block::Block;

use crate::{bitreader::Readable, ParseError};

use self::tagged_bit_reader::TaggedBitreader;

/// Simplified parsing method only accepting reader
pub trait TypeParse {
    fn parse(reader: &mut TaggedBitreader<impl Readable>) -> Result<Self, ParseError>
    where
        Self: Sized;
}
