mod block;
mod crdt;
mod lwwvalue;
mod scene_item;
mod scene_tree;
mod subblock;

mod tagged_bit_reader;

pub use block::{blocks, Block, BlockParse};
pub use scene_tree::SceneTree;
pub use tagged_bit_reader::TaggedBitreader;

use crate::{bitreader::Readable, ParseError};

/// Simplified parsing method only accepting reader
pub trait TypeParse {
    fn parse(reader: &mut TaggedBitreader<impl Readable>) -> Result<Self, ParseError>
    where
        Self: Sized;
}
