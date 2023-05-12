mod scene_tree;
mod tagged_bit_reader;
pub use scene_tree::SceneTree;

mod block;
pub use block::Block;

use crate::{bitreader::Readable, ParseError};

mod tag;

mod crdt;

/// Simplified parsing method only accepting reader
pub trait TypeParse {
    fn parse(reader: &mut crate::Bitreader<impl Readable>) -> Result<Self, ParseError>
    where
        Self: Sized;
}
