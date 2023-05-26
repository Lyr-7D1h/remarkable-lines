use crate::{bitreader::Readable, Bitreader, ParseError};

pub mod layer;
pub mod line;
pub mod page;
pub mod point;

pub use page::Page;

pub trait Parse {
    fn parse(version: u32, reader: &mut Bitreader<impl Readable>) -> Result<Self, ParseError>
    where
        Self: Sized;
}
