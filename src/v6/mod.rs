pub mod block;
pub mod crdt;
pub mod lwwvalue;
pub mod scene_item;
pub mod scene_tree;

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
