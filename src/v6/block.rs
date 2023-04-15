use std::io::Read;

use crate::{Bitreader, Parse, ParseError};

mod blocks;
mod crdtid;
mod tag;
use blocks::*;

#[derive(Debug)]
pub struct BlockInfo {
    start_offset: usize,
    size: u32,
    min_version: u8,
    current_version: u8,
}

impl BlockInfo {
    pub fn has_bytes_remaining(&self, reader: &Bitreader<impl Read>) -> bool {
        self.start_offset + self.size as usize > reader.offset()
    }
}

#[derive(Debug)]
pub enum Block {
    MigrationInfo(MigrationInfoBlock),
    PageInfo(PageInfoBlock),
    TreeNode(TreeNodeBlock),
    SceneTree(SceneTreeBlock),
    SceneGlyphItem,
    SceneGroupItem,
    SceneLineItem,
    SceneTextItem,
    AuthorsIds(AuthorsIdsBlock),
    RootText,
}

/// Simplified parsing method only accepting reader
pub trait TypeParse {
    fn parse<N: Read>(reader: &mut crate::Bitreader<N>) -> Result<Self, ParseError>
    where
        Self: Sized;
}
/// Parsing methods for parsing blocks
pub trait BlockParse {
    fn parse<N: Read>(
        info: &BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl Parse for Block {
    fn parse<N: std::io::Read>(
        version: u32,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        let size = reader.read_u32()?;
        // unknown value
        let _ = reader.read_u8()?;
        let min_version = reader.read_u8()?;
        let current_version = reader.read_u8()?;
        let block_type = reader.read_u8()?;

        if current_version < min_version {
            return Err(ParseError::invalid(
                "current_version can't be smaller than min_version",
            ));
        }

        let start_offset = reader.offset();

        println!(
            "Starting new block at offset {:x} until {:x}",
            reader.offset() - 4,
            start_offset + size as usize
        );

        let info = BlockInfo {
            start_offset,
            size,
            min_version,
            current_version,
        };

        let block = match block_type {
            0x00 => Block::MigrationInfo(MigrationInfoBlock::parse(&info, reader)?),
            0x01 => Block::SceneTree(SceneTreeBlock::parse(&info, reader)?),
            0x02 => Block::TreeNode(TreeNodeBlock::parse(&info, reader)?),
            0x03 => Block::SceneGlyphItem,
            0x04 => Block::SceneGroupItem,
            0x05 => Block::SceneLineItem,
            0x06 => Block::SceneTextItem,
            0x07 => Block::RootText,
            0x09 => Block::AuthorsIds(AuthorsIdsBlock::parse(&info, reader)?),
            0x0A => Block::PageInfo(PageInfoBlock::parse(&info, reader)?),
            _ => {
                return Err(ParseError::invalid(format!(
                    "Unknown block type: '{block_type}'"
                )))
            }
        };

        let expected_offset = start_offset + usize::try_from(size)?;
        let end_offset = reader.offset();
        if expected_offset != reader.offset() {
            return Err(ParseError::invalid(format!(
                "Block type '{block_type}' did not read expected size. Expected {expected_offset:x} given {end_offset:x}" 
            )));
        }

        return Ok(block);
    }
}
